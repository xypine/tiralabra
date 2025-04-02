use std::collections::{BinaryHeap, HashMap};

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::{
    interface::{GridInterface, TileInterface},
    rules::RuleSet,
    tile::{Tile, TileState},
    utils::{
        entropy::EntropyHeapEntry,
        space::{Delta2D, Direction2D, Location2D, NEIGHBOUR_COUNT_2D},
    },
};

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DynamicSizeGrid2D {
    #[tsify(type = "RuleSet<Direction2D>")]
    pub rules: RuleSet<NEIGHBOUR_COUNT_2D, Direction2D>,
    pub width: usize,
    pub height: usize,
    tiles: Vec<Tile>,
    // Priority queue based on tile entropy
    #[tsify(type = "any")]
    entropy_heap: BinaryHeap<EntropyHeapEntry>,
    // Used to invalidate entries in the entropy_heap
    entropy_invalidation_matrix: Vec<usize>,
}

impl DynamicSizeGrid2D {
    fn update_tile(&mut self, location: Location2D, state: Tile) -> Option<()> {
        let current_state = self.get_tile(location)?;

        if state == current_state {
            // no update needed
            return Some(());
        }

        let tile_index = self.location_to_index(location);
        self.tiles[tile_index] = state;
        self.update_tile_entropy(location);

        Some(())
    }

    #[inline]
    fn update_tile_entropy(&mut self, location: Location2D) {
        let matrix_index = self.location_to_index(location);

        let current_version = self.entropy_invalidation_matrix[matrix_index];
        let new_version = current_version + 1;

        self.entropy_invalidation_matrix[matrix_index] = new_version;
        if let Some(new_entropy) = self.tiles[matrix_index].calculate_entropy() {
            self.entropy_heap.push(EntropyHeapEntry {
                location,
                entropy: new_entropy,
                version: new_version,
            });
        } else {
            // no updated version is pushed, so it's impossible for the tile to be picked
        }
    }

    pub fn dump(&self) -> Vec<Tile> {
        self.tiles.clone()
    }
}

impl DynamicSizeGrid2D {
    pub fn new(
        width: usize,
        height: usize,
        rules: RuleSet<NEIGHBOUR_COUNT_2D, Direction2D>,
    ) -> Self {
        let tiles = vec![Tile::new(rules.possible.clone()); width * height];
        let tile_invalidation_matrix = vec![0; width * height];
        let mut new = Self {
            width,
            height,
            rules,
            tiles,
            entropy_heap: BinaryHeap::new(),
            entropy_invalidation_matrix: tile_invalidation_matrix,
        };

        for x in 0..width {
            for y in 0..height {
                new.update_tile_entropy(Location2D { x, y });
            }
        }

        new
    }

    #[inline]
    fn index_to_location(&self, i: usize) -> Location2D {
        let x = i % self.width;
        let y = i / self.width;
        Location2D { x, y }
    }

    #[inline]
    fn location_to_index(&self, location: Location2D) -> usize {
        location.y * self.width + location.x
    }
}

impl GridInterface<4, TileState, Location2D, Direction2D, Tile> for DynamicSizeGrid2D {
    fn image(&self) -> std::collections::HashMap<Location2D, Tile> {
        let mut map = HashMap::new();
        for (i, tile) in self.tiles.iter().enumerate() {
            let position = self.index_to_location(i);
            map.insert(position, tile.clone());
        }
        map
    }

    fn get_tile(&self, location: Location2D) -> Option<Tile> {
        let index = location.y * self.width + location.x;
        self.tiles.get(index).cloned()
    }

    fn get_neighbours(&self, location: Location2D) -> [(Direction2D, Option<Location2D>); 4] {
        std::array::from_fn(|index| {
            let direction = Direction2D::try_from(index).unwrap();
            let direction_delta = Delta2D::from(direction);
            let location = if let Ok(neighbour_location) = location.try_apply(direction_delta) {
                if neighbour_location.x >= self.width || neighbour_location.y >= self.height {
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

    fn get_lowest_entropy_position(&mut self) -> Option<Location2D> {
        if let Some(candidate) = self.entropy_heap.peek() {
            let candidate_index = self.location_to_index(candidate.location);
            let current_version = self.entropy_invalidation_matrix[candidate_index];
            if candidate.version < current_version {
                //println!(
                //    "candidate {:?} was outdated (latest {current_version})",
                //    candidate
                //);
                let _ = self.entropy_heap.pop();
                return self.get_lowest_entropy_position();
            }
            // println!("PICKED {:?}", candidate);
            return Some(candidate.location);
        }
        None
    }

    fn with_tile<R, F: Fn(&mut Tile) -> R>(&mut self, location: Location2D, f: F) -> Option<R> {
        let mut mutable_copy = self.get_tile(location)?;
        let result = f(&mut mutable_copy);
        self.update_tile(location, mutable_copy)?;
        Some(result)
    }

    fn get_neighbour_tiles(&self, location: Location2D) -> [(Direction2D, Option<Tile>); 4] {
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

    fn get_rules(&self) -> RuleSet<4, Direction2D> {
        self.rules.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashSet};

    use crate::interface::TileInterface;

    use super::*;

    fn debug_print(grid: &DynamicSizeGrid2D) {
        for y in 0..grid.height {
            for x in 0..grid.width {
                let tile = grid.get_tile(Location2D { x, y }).unwrap();
                let id = tile.possible_states().next().unwrap();
                print!("{id: <4}")
            }
            println!()
        }
        println!()
    }
    fn id(position: Location2D, _w: usize, h: usize) -> TileState {
        (position.y * h + position.x) as u64
    }

    fn init_and_check(possible: BTreeSet<TileState>, w: usize, h: usize) -> DynamicSizeGrid2D {
        let allowed = HashSet::from([]);
        let rules = RuleSet::new(possible, allowed);
        let grid = DynamicSizeGrid2D::new(w, h, rules);
        assert_eq!(grid.tiles.len(), w * h);

        grid
    }

    fn init_id(w: usize, h: usize) -> DynamicSizeGrid2D {
        let mut grid = init_and_check(BTreeSet::new(), w, h);
        for x in 0..w {
            for y in 0..h {
                let location = Location2D { x, y };
                let unique = id(location, w, h);
                let index = grid.location_to_index(location);
                grid.tiles[index].set_possible_states(BTreeSet::from([unique]));
            }
        }

        grid
    }

    #[test]
    fn init() {
        init_and_check(BTreeSet::new(), 3, 3);
    }

    #[test]
    fn init_asymmetric() {
        init_and_check(BTreeSet::new(), 3, 4);
    }

    #[test]
    fn init_and_image() {
        const W: usize = 5;
        const H: usize = 3;
        let init_possible: BTreeSet<TileState> = BTreeSet::from([0, 1, 2, 3]);
        let grid = init_and_check(init_possible.clone(), W, H);
        let image = grid.image();
        (0..W).for_each(|x| {
            (0..H).for_each(|y| {
                let tile = image
                    .get(&Location2D { x, y })
                    .expect("failed to access tile");
                let tile_possible = BTreeSet::from_iter(tile.possible_states());
                assert_eq!(tile_possible, init_possible);
            });
        });
    }

    #[test]
    fn init_and_access() {
        const W: usize = 4;
        const H: usize = 6;
        let grid = init_id(W, H);
        debug_print(&grid);

        crate::grid::tests::get_tile(W, H, grid);
    }

    #[test]
    fn get_neighbours_sanity() {
        const W: usize = 3;
        const H: usize = 3;
        let grid = init_id(W, H);
        debug_print(&grid);

        crate::grid::tests::get_neighbours_sanity(W, H, grid);
    }
}
