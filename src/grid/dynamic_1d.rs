//! A Grid that can be initialized at any size
//!

use std::collections::{BinaryHeap, HashMap};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::{
    rules::RuleSet,
    tile::{Tile, TileState, interface::TileInterface},
    utils::{
        entropy::EntropyHeapEntry1D,
        space::s1d::{Delta1D, Direction1D, Location1D, NEIGHBOUR_COUNT_1D},
    },
};

use super::GridInterface;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DynamicSizeGrid1D {
    #[tsify(type = "RuleSet<Direction1D>")]
    pub rules: RuleSet<NEIGHBOUR_COUNT_1D, Direction1D>,
    pub width: usize,
    // A one dimensional array is used for potentionally better performance
    // (cache locality, fewer bounds checks - if enabled)
    tiles: Vec<Tile>,
    /// Priority queue based on tile entropy
    #[tsify(type = "any")]
    entropy_heap: BinaryHeap<EntropyHeapEntry1D>,
    /// Used to invalidate entries in the entropy_heap
    entropy_invalidation_matrix: Vec<usize>,
    /// Keeps history of tile modifications for backtracking
    pub update_log: Vec<(Location1D, Tile)>,
    /// Dictates random events
    rng: ChaCha8Rng,
}

impl DynamicSizeGrid1D {
    /// Updates a tile at the given location and it's entry in the entropy heap
    fn update_tile(&mut self, location: Location1D, state: Tile) -> Option<()> {
        let current_state = self.get_tile(location)?;

        if state == *current_state {
            // no update needed
            return Some(());
        }

        let tile_index = self.location_to_index(location);
        self.tiles[tile_index] = state.clone();
        self.update_tile_entropy(location);
        self.update_log.push((location, state));

        Some(())
    }

    /// Calculates an entropy for the tile at the given location
    ///
    /// If the value has changed the last time, the current entry is invalidated and a new one is
    /// inserted
    #[inline]
    fn update_tile_entropy(&mut self, location: Location1D) {
        let matrix_index = self.location_to_index(location);

        let current_version = self.entropy_invalidation_matrix[matrix_index];
        let new_version = current_version + 1;

        self.entropy_invalidation_matrix[matrix_index] = new_version;
        if let Some(new_entropy) =
            self.tiles[matrix_index].calculate_entropy(&self.rules.weights, &mut self.rng)
        {
            self.entropy_heap.push(EntropyHeapEntry1D {
                location,
                entropy: new_entropy,
                version: new_version,
            });
        } else {
            // no updated version is pushed, so it becomes impossible for the tile to be picked
        }
    }

    pub fn tiles_ref(&self) -> &Vec<Tile> {
        &self.tiles
    }
}

impl DynamicSizeGrid1D {
    pub fn new(
        width: usize,
        rules: RuleSet<NEIGHBOUR_COUNT_1D, Direction1D>,
        rng_seed: u64,
    ) -> Self {
        let tiles = vec![Tile::new(rules.possible.clone()); width];
        let tile_invalidation_matrix = vec![0; width];
        let mut new = Self {
            width,
            rules: rules.clone(),
            tiles,
            entropy_heap: BinaryHeap::new(),
            entropy_invalidation_matrix: tile_invalidation_matrix,
            update_log: Vec::new(),
            rng: ChaCha8Rng::seed_from_u64(rng_seed),
        };

        if !rules.initialize_edges.is_empty() {
            unimplemented!();
        }

        for x in 0..width {
            new.update_tile_entropy(Location1D { x });
        }

        new
    }

    #[inline]
    fn index_to_location(&self, i: usize) -> Location1D {
        Location1D { x: i }
    }

    #[inline]
    fn location_to_index(&self, location: Location1D) -> usize {
        location.x
    }
}

// See `GridInterface` for further documentation
impl GridInterface<NEIGHBOUR_COUNT_1D, TileState, Location1D, Direction1D, Tile>
    for DynamicSizeGrid1D
{
    fn get_dimensions(&self) -> Location1D {
        Location1D { x: self.width }
    }

    fn reset(&mut self) {
        *self = Self::new(self.width, self.rules.clone(), self.rng.random())
    }

    fn image(&self) -> std::collections::HashMap<Location1D, Tile> {
        let mut map = HashMap::new();
        for (i, tile) in self.tiles.iter().enumerate() {
            let position = self.index_to_location(i);
            map.insert(position, tile.clone());
        }
        map
    }

    fn get_tile(&self, location: Location1D) -> Option<&Tile> {
        let index = self.location_to_index(location);
        self.tiles.get(index)
    }

    fn get_tiles_at_time(&self, time_index: usize) -> HashMap<Location1D, Tile> {
        let mut tiles = HashMap::new();
        let mut i = 0;
        for (location, new_state) in &self.update_log {
            tiles.insert(*location, new_state.clone());
            i += 1;
            if i > time_index {
                break;
            }
        }
        tiles
    }

    fn get_neighbours(
        &self,
        location: Location1D,
    ) -> [(Direction1D, Option<Location1D>); NEIGHBOUR_COUNT_1D] {
        // index is 0..4
        std::array::from_fn(|index| {
            let direction = Direction1D::try_from(index).unwrap();
            let direction_delta = Delta1D::from(direction);
            let location = if let Ok(neighbour_location) = location.try_apply(direction_delta) {
                if neighbour_location.x >= self.width {
                    None
                } else {
                    Some(neighbour_location)
                }
            } else {
                None
            };
            (direction, location)
        })
    }

    fn get_neighbour_tiles(
        &self,
        location: Location1D,
    ) -> [(Direction1D, Option<&Tile>); NEIGHBOUR_COUNT_1D] {
        let locations = self.get_neighbours(location);
        std::array::from_fn(|index| {
            let (direction, neighbour_location) = locations[index];
            let neighbour = if let Some(neighbour_location) = neighbour_location {
                self.get_tile(neighbour_location)
            } else {
                None
            };
            (direction, neighbour)
        })
    }

    fn get_lowest_entropy_position(&mut self) -> Option<Location1D> {
        if let Some(candidate) = self.entropy_heap.peek() {
            let candidate_index = self.location_to_index(candidate.location);
            let current_version = self.entropy_invalidation_matrix[candidate_index];
            // My implementation for invalidating entries in the entropy heap requires some
            // extra work.
            // Instead of removing entries from the heap when new versions come in, we ignore them
            // at access time
            if candidate.version < current_version {
                let _ = self.entropy_heap.pop();
                // this means that the access call can be recursive - at worst we need to scan and
                // discard the entire heap
                return self.get_lowest_entropy_position();
            }
            return Some(candidate.location);
        }
        None
    }

    fn with_tile<R, F: Fn(&mut Tile, &mut ChaCha8Rng) -> R>(
        &mut self,
        location: Location1D,
        f: F,
    ) -> Option<R> {
        // give the caller mutable access to a copied version of the tile
        let mut mutable_copy = self.get_tile(location)?.clone();
        let result = f(&mut mutable_copy, &mut self.rng);
        // update the actual tile, updating the entropy heap if needed
        self.update_tile(location, mutable_copy)?;
        Some(result)
    }

    fn get_rules(&self) -> &RuleSet<NEIGHBOUR_COUNT_1D, Direction1D> {
        &self.rules
    }
}
