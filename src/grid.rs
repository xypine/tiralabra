use std::collections::{HashMap, HashSet};

use crate::{
    interface::GridInterface,
    space::{Delta2D, Direction2D, Location2D},
    tile::{Tile, TileState},
};

pub struct ConstantSizeGrid<const W: usize, const H: usize> {
    tiles: [[Tile; H]; W],
}

impl<const W: usize, const H: usize> ConstantSizeGrid<W, H> {
    pub fn new(possible_tile_states: HashSet<TileState>) -> Self {
        let tiles = std::array::from_fn(|_| {
            std::array::from_fn(|_| Tile::new(possible_tile_states.clone()))
        });
        Self { tiles }
    }
}

impl<const W: usize, const H: usize> GridInterface<2, TileState, Location2D, Direction2D, Tile>
    for ConstantSizeGrid<W, H>
{
    fn get_tile(&self, location: Location2D) -> Option<Tile> {
        self.tiles
            .get(location.x)
            .and_then(|col| col.get(location.y))
            .cloned()
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

    fn get_neighbours(&self, location: Location2D) -> [(Direction2D, Option<Tile>); 2 * 2] {
        std::array::from_fn(|index| {
            let direction = Direction2D::try_from(index).unwrap();
            let direction_delta = Delta2D::from(direction);
            let neighbour = if let Some(neighbour_location) =
                location.try_apply(direction_delta).ok()
            {
                println!(
                    "{location:?} + {direction:?} ({direction_delta:?}) = {neighbour_location:?}"
                );
                self.get_tile(neighbour_location)
            } else {
                None
            };
            (direction, neighbour)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::interface::TileInterface;

    use super::*;

    fn debug_print<const W: usize, const H: usize>(grid: &ConstantSizeGrid<W, H>) {
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
        possible: HashSet<TileState>,
    ) -> ConstantSizeGrid<W, H> {
        let grid: ConstantSizeGrid<W, H> = ConstantSizeGrid::new(possible);
        assert_eq!(grid.tiles.len(), W);
        for col in &grid.tiles {
            assert_eq!(col.len(), H);
        }

        grid
    }

    fn init_id<const W: usize, const H: usize>() -> ConstantSizeGrid<W, H> {
        let mut grid = init_and_check::<W, H>(HashSet::new());
        for x in 0..W {
            for y in 0..H {
                let unique = id(Location2D { x, y }, W, H);
                grid.tiles[x][y].force_set(unique);
            }
        }

        grid
    }

    #[test]
    fn init() {
        init_and_check::<3, 3>(HashSet::new());
    }

    #[test]
    fn init_asymmetric() {
        init_and_check::<3, 4>(HashSet::new());
    }

    #[test]
    fn init_and_image() {
        const W: usize = 5;
        const H: usize = 3;
        let init_possible: HashSet<TileState> = HashSet::from([0, 1, 2, 3]);
        let grid = init_and_check::<W, H>(init_possible.clone());
        let image = grid.image();
        (0..W).for_each(|x| {
            (0..H).for_each(|y| {
                let tile = image
                    .get(&Location2D { x, y })
                    .expect("failed to access tile");
                let tile_possible = HashSet::from_iter(tile.possible_states());
                assert_eq!(tile_possible, init_possible);
            });
        });
    }

    fn assert_tile_state(tile: &Tile, expected: TileState) {
        let mut tile_possible = tile.possible_states();
        assert_eq!(
            tile_possible
                .next()
                .expect("tile should've been initialized with one possible state"),
            expected,
            "tile state didn't match expectations"
        );
        assert!(
            tile_possible.next().is_none(),
            "tile should've been initialized with one possible state",
        )
    }

    #[test]
    fn init_and_access() {
        const W: usize = 4;
        const H: usize = 6;
        let grid = init_id::<W, H>();
        debug_print(&grid);
        let mut seen_ids = HashSet::new();
        (0..W).for_each(|x| {
            (0..H).for_each(|y| {
                let tile = grid
                    .get_tile(Location2D { x, y })
                    .expect("get_tile should succeed inside W and H");
                let unique = id(Location2D { x, y }, W, H);

                assert_tile_state(&tile, unique);

                println!("adding {unique} from ({x}, {y})");
                println!("{:?}", seen_ids);

                let tile_is_unique = !seen_ids.contains(&unique);
                assert!(tile_is_unique);

                seen_ids.insert(unique);
            });
        });
        (W..(W * 2)).for_each(|x| {
            (H..(H * 2)).for_each(|y| {
                let tile = grid.get_tile(Location2D { x, y });
                assert!(tile.is_none(), "get_tile should fail outside W and H");
            });
        });
    }

    #[test]
    fn get_neighbours_sanity() {
        const W: usize = 3;
        const H: usize = 3;
        let grid = init_id::<W, H>();
        debug_print(&grid);

        let test_tile_neighbours =
            |location: Location2D, expected_neighbours: [Option<Location2D>; 4]| {
                let our_id = id(location, W, H);
                let tile = grid.get_tile(location).unwrap();
                assert_tile_state(&tile, our_id);

                let mut expected_neighbour_ids = vec![];
                for neighbour_location in expected_neighbours {
                    if let Some(neighbour_location) = neighbour_location {
                        let neighbour_id = id(neighbour_location, W, H);
                        let neighbour = grid.get_tile(neighbour_location).unwrap();
                        assert_tile_state(&neighbour, neighbour_id);
                        expected_neighbour_ids.push(Some(neighbour_id));
                    } else {
                        expected_neighbour_ids.push(None);
                    }
                }

                let impl_neighbours = grid.get_neighbours(location);
                for (i, (dir, impl_neighbour)) in impl_neighbours.iter().cloned().enumerate() {
                    println!("=== Neighbour {i} ===");
                    println!("impl resolved to direction {dir:?}");
                    if let Some(reference_id) = expected_neighbour_ids[i] {
                        let impl_neighbour =
                            impl_neighbour.expect("get_neighbours missing neighbour");
                        assert_tile_state(&impl_neighbour, reference_id);
                    } else {
                        assert!(impl_neighbour.is_none())
                    }
                }
            };

        // Middle tile
        test_tile_neighbours(
            Location2D { x: 1, y: 1 },
            [
                Some(Location2D { x: 1, y: 0 }),
                Some(Location2D { x: 2, y: 1 }),
                Some(Location2D { x: 1, y: 2 }),
                Some(Location2D { x: 0, y: 1 }),
            ],
        );
    }
}
