//! A Grid which size is known at compile-time
//!
//! Not very practical for most applications (where the size of the grid _may_ be determined at
//! runtime). This was the initial version used for testing and reasoning. It might have a bit
//! better performance when compared to a dynamically allocated version.

use std::collections::{BinaryHeap, HashMap, VecDeque};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{
    rules::RuleSet,
    tile::{Tile, TileState, interface::TileInterface},
    utils::{
        entropy::EntropyHeapEntry,
        space::s2d::{Delta2D, Direction2D, Location2D, NEIGHBOUR_COUNT_2D},
    },
    wave_function_collapse::{interface::WaveFunctionCollapse, propagate_from_tile},
};

use super::GridInterface;

#[derive(Debug)]
pub struct ConstantSizeGrid2D<const W: usize, const H: usize> {
    pub rules: RuleSet<NEIGHBOUR_COUNT_2D, Direction2D>,
    tiles: [[Tile; H]; W],
    /// Priority queue based on tile entropy
    entropy_heap: BinaryHeap<EntropyHeapEntry>,
    /// Used to invalidate entries in the entropy_heap
    entropy_invalidation_matrix: [[usize; H]; W],

    /// Dictates random events
    rng: ChaCha8Rng,
}
impl<const W: usize, const H: usize> ConstantSizeGrid2D<W, H> {
    fn update_tile(&mut self, location: Location2D, state: Tile) -> Option<()> {
        let current_state = self.get_tile(location)?;

        if state == *current_state {
            // no update needed
            return Some(());
        }

        self.tiles[location.x][location.y] = state;
        self.update_tile_entropy(location);

        Some(())
    }

    #[inline]
    fn update_tile_entropy(&mut self, location: Location2D) {
        let current_version = self.entropy_invalidation_matrix[location.x][location.y];
        let new_version = current_version + 1;
        //println!("UPD {location:?}v{new_version} = {new_entropy:?}");
        self.entropy_invalidation_matrix[location.x][location.y] = new_version;
        if let Some(new_entropy) =
            self.tiles[location.x][location.y].calculate_entropy(&self.rules.weights, &mut self.rng)
        {
            self.entropy_heap.push(EntropyHeapEntry {
                location,
                entropy: new_entropy,
                version: new_version,
            });
        } else {
            // no updated version is pushed, so it's impossible for the tile to be picked
        }
    }
}

impl<const W: usize, const H: usize> ConstantSizeGrid2D<W, H> {
    pub fn new(rules: RuleSet<NEIGHBOUR_COUNT_2D, Direction2D>, rng_seed: u64) -> Self {
        let tiles =
            std::array::from_fn(|_| std::array::from_fn(|_| Tile::new(rules.possible.clone())));
        let tile_invalidation_matrix = std::array::from_fn(|_| std::array::from_fn(|_| 0));
        let mut new = Self {
            rules: rules.clone(),
            tiles,
            entropy_heap: BinaryHeap::new(),
            entropy_invalidation_matrix: tile_invalidation_matrix,
            rng: ChaCha8Rng::seed_from_u64(rng_seed),
        };

        let mut initial_propagation_queue = VecDeque::new();
        for (direction, tile_state) in &rules.initialize_edges {
            let edge_tile_locations: Vec<_> = match Delta2D::from(*direction) {
                Delta2D { x: dx, y: 0 } => {
                    let x = if dx > 0 { W - 1 } else { 0 };
                    (0..H).map(|y| Location2D { x, y }).collect()
                }
                Delta2D { x: 0, y: dy } => {
                    let y = if dy > 0 { H - 1 } else { 0 };
                    (0..W).map(|x| Location2D { x, y }).collect()
                }
                _ => unreachable!(),
            };
            for location in edge_tile_locations {
                new.with_tile(location, |t, _| {
                    t.set_possible_states([*tile_state]);
                });
                initial_propagation_queue.extend(propagate_from_tile(&new, location));
            }
        }

        new.propagate(initial_propagation_queue).expect(
            "Propagation got interrupted after an edge was collapsed, please revise your ruleset",
        );

        for x in 0..W {
            for y in 0..H {
                new.update_tile_entropy(Location2D { x, y });
            }
        }

        new
    }
}

impl<const W: usize, const H: usize> GridInterface<4, TileState, Location2D, Direction2D, Tile>
    for ConstantSizeGrid2D<W, H>
{
    fn get_dimensions(&self) -> Location2D {
        Location2D { x: W, y: H }
    }

    fn reset(&mut self) {
        *self = Self::new(self.rules.clone(), self.rng.random())
    }

    fn image(&self) -> std::collections::HashMap<Location2D, Tile> {
        let mut map = HashMap::new();
        for (x, col) in self.tiles.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                let position = Location2D { x, y };
                map.insert(position, tile.clone());
            }
        }
        map
    }

    fn get_tile(&self, location: Location2D) -> Option<&Tile> {
        self.tiles
            .get(location.x)
            .and_then(|col| col.get(location.y))
    }

    fn get_neighbours(&self, location: Location2D) -> [(Direction2D, Option<Location2D>); 4] {
        std::array::from_fn(|index| {
            let direction = Direction2D::try_from(index).unwrap();
            let direction_delta = Delta2D::from(direction);
            let location = if let Ok(neighbour_location) = location.try_apply(direction_delta) {
                if neighbour_location.x >= W || neighbour_location.y >= H {
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

    fn get_neighbour_tiles(&self, location: Location2D) -> [(Direction2D, Option<&Tile>); 4] {
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

    fn get_lowest_entropy_position(&mut self) -> Option<Location2D> {
        if let Some(candidate) = self.entropy_heap.peek() {
            let current_version =
                self.entropy_invalidation_matrix[candidate.location.x][candidate.location.y];
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
        location: Location2D,
        f: F,
    ) -> Option<R> {
        // give the caller mutable access to a copied version of the tile
        let mut mutable_copy = self.get_tile(location)?.clone();
        let result = f(&mut mutable_copy, &mut self.rng);
        // update the actual tile, updating the entropy heap if needed
        self.update_tile(location, mutable_copy)?;
        Some(result)
    }

    fn get_rules(&self) -> &RuleSet<4, Direction2D> {
        &self.rules
    }

    fn positions(&self) -> impl Iterator<Item = Location2D> {
        (0..W).flat_map(|x| (0..H).map(move |y| Location2D { x, y }))
    }

    fn get_tiles_at_time(&self, _time_index: usize) -> HashMap<Location2D, Tile> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet, HashSet};

    use super::*;

    fn debug_print<const W: usize, const H: usize>(grid: &ConstantSizeGrid2D<W, H>) {
        for y in 0..H {
            for x in 0..W {
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

    fn init_and_check<const W: usize, const H: usize>(
        possible: BTreeSet<TileState>,
    ) -> ConstantSizeGrid2D<W, H> {
        let allowed = HashSet::from([]);
        let rules = RuleSet::new(
            possible,
            allowed,
            HashMap::new(),
            HashMap::new(),
            BTreeMap::new(),
        );
        let grid: ConstantSizeGrid2D<W, H> = ConstantSizeGrid2D::new(rules, 0);
        assert_eq!(grid.tiles.len(), W);
        for col in &grid.tiles {
            assert_eq!(col.len(), H);
        }

        grid
    }

    fn init_id<const W: usize, const H: usize>() -> ConstantSizeGrid2D<W, H> {
        let mut grid = init_and_check::<W, H>(BTreeSet::new());
        for x in 0..W {
            for y in 0..H {
                let unique = id(Location2D { x, y }, W, H);
                grid.tiles[x][y].set_possible_states(BTreeSet::from([unique]));
            }
        }

        grid
    }

    #[test]
    fn init() {
        init_and_check::<3, 3>(BTreeSet::new());
    }

    #[test]
    fn init_asymmetric() {
        init_and_check::<3, 4>(BTreeSet::new());
    }

    #[test]
    fn init_and_image() {
        const W: usize = 5;
        const H: usize = 3;
        let init_possible: BTreeSet<TileState> = BTreeSet::from([0, 1, 2, 3]);
        let grid = init_and_check::<W, H>(init_possible.clone());
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
        let grid = init_id::<W, H>();
        debug_print(&grid);

        crate::grid::tests::get_tile(W, H, grid);
    }

    #[test]
    fn get_neighbours_sanity() {
        const W: usize = 3;
        const H: usize = 3;
        let grid = init_id::<W, H>();
        debug_print(&grid);

        crate::grid::tests::get_neighbours_sanity(W, H, grid);
    }

    #[test]
    fn entropy_heap_empty() {
        const W: usize = 0;
        const H: usize = 0;
        let mut grid = init_id::<W, H>();

        assert!(grid.get_lowest_entropy_position().is_none());
    }

    #[test]
    fn update_tiles() {
        const W: usize = 3;
        const H: usize = 3;
        let mut grid = init_id::<W, H>();
        debug_print(&grid);

        crate::grid::tests::update_tiles_sanity(W, H, &mut grid);
    }

    #[test]
    fn update_entropy() {
        const W: usize = 3;
        const H: usize = 3;
        let mut grid = init_id::<W, H>();
        debug_print(&grid);

        crate::grid::tests::update_tiles_entropy(W, H, &mut grid);
    }

    #[test]
    fn edge_initialization_2x2() {
        crate::grid::tests::edges_2x2(|rules| ConstantSizeGrid2D::<2, 2>::new(rules, 0));
    }
}
