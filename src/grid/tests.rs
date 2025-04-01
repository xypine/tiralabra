use std::collections::HashSet;

use crate::{
    interface::{GridInterface, TileInterface},
    tile::{Tile, TileState},
    utils::space::{Direction2D, Location2D, NEIGHBOUR_COUNT_2D},
};

fn id(position: Location2D, _w: usize, h: usize) -> TileState {
    (position.y * h + position.x) as u64
}

pub fn assert_tile_state(tile: &Tile, expected: TileState) {
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

pub fn get_tile<T: GridInterface<NEIGHBOUR_COUNT_2D, TileState, Location2D, Direction2D, Tile>>(
    w: usize,
    h: usize,
    grid: T,
) {
    let mut seen_ids = HashSet::new();
    (0..w).for_each(|x| {
        (0..h).for_each(|y| {
            let tile = grid
                .get_tile(Location2D { x, y })
                .expect("get_tile should succeed inside W and H");
            let unique = id(Location2D { x, y }, w, h);

            assert_tile_state(&tile, unique);

            println!("adding {unique} from ({x}, {y})");
            println!("{:?}", seen_ids);

            let tile_is_unique = !seen_ids.contains(&unique);
            assert!(tile_is_unique);

            seen_ids.insert(unique);
        });
    });
    (w..(w * 2)).for_each(|x| {
        (h..(h * 2)).for_each(|y| {
            let tile = grid.get_tile(Location2D { x, y });
            assert!(tile.is_none(), "get_tile should fail outside W and H");
        });
    });
}

pub fn get_neighbours_sanity<
    T: GridInterface<NEIGHBOUR_COUNT_2D, TileState, Location2D, Direction2D, Tile>,
>(
    w: usize,
    h: usize,
    grid: T,
) {
    let test_tile_neighbours =
        |location: Location2D, expected_neighbours: [Option<Location2D>; 4]| {
            let our_id = id(location, w, h);
            let tile = grid.get_tile(location).unwrap();
            assert_tile_state(&tile, our_id);

            let mut expected_neighbour_ids = vec![];
            for neighbour_location in expected_neighbours {
                if let Some(neighbour_location) = neighbour_location {
                    let neighbour_id = id(neighbour_location, w, h);
                    let neighbour = grid.get_tile(neighbour_location).unwrap();
                    assert_tile_state(&neighbour, neighbour_id);
                    expected_neighbour_ids.push(Some(neighbour_id));
                } else {
                    expected_neighbour_ids.push(None);
                }
            }

            let impl_neighbours = grid.get_neighbour_tiles(location);
            for (i, (dir, impl_neighbour)) in impl_neighbours.iter().cloned().enumerate() {
                println!("=== Neighbour {i} ===");
                println!("impl resolved to direction {dir:?}");
                if let Some(reference_id) = expected_neighbour_ids[i] {
                    let impl_neighbour = impl_neighbour.expect("get_neighbours missing neighbour");
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
