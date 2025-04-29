//! Common test helpers for grid-related tests

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::{
    rules::RuleSet2D,
    tile::{Tile, TileState, interface::TileInterface},
    utils::space::s2d::{Direction2D, Location2D, NEIGHBOUR_COUNT_2D},
};

use super::GridInterface;

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

    // top-left corner
    test_tile_neighbours(
        Location2D { x: 0, y: 0 },
        [
            None,
            Some(Location2D { x: 1, y: 0 }),
            Some(Location2D { x: 0, y: 1 }),
            None,
        ],
    );

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

    // bottom-right corner
    test_tile_neighbours(
        Location2D { x: 2, y: 2 },
        [
            Some(Location2D { x: 2, y: 1 }),
            None,
            None,
            Some(Location2D { x: 1, y: 2 }),
        ],
    );
}

pub fn update_tiles_sanity<
    T: GridInterface<NEIGHBOUR_COUNT_2D, TileState, Location2D, Direction2D, Tile>,
>(
    w: usize,
    h: usize,
    grid: &mut T,
) {
    for position in [
        Location2D { x: 0, y: 0 },
        Location2D { x: w - 1, y: 0 },
        Location2D { x: 0, y: h - 1 },
        Location2D { x: w - 1, y: h - 1 },
    ] {
        let expected_initial = BTreeSet::from([0, 1, 2, 3]);
        grid.with_tile(position, |t, _| {
            t.set_possible_states(expected_initial.clone())
        });
        let impl_initial = BTreeSet::from_iter(
            grid.get_tile(position)
                .expect("failed to access tile at test position")
                .possible_states(),
        );
        assert_eq!(impl_initial, expected_initial);
        let expected_after_modification = BTreeSet::from([0, 1, 2]);
        grid.with_tile(position, |t, _| {
            t.set_possible_states(expected_after_modification.clone())
        });
        let impl_after_modification = BTreeSet::from_iter(
            grid.get_tile(position)
                .expect("failed to access tile at test position")
                .possible_states(),
        );
        assert_eq!(impl_after_modification, expected_after_modification);
    }
}

pub fn update_tiles_entropy<
    T: GridInterface<NEIGHBOUR_COUNT_2D, TileState, Location2D, Direction2D, Tile>,
>(
    w: usize,
    h: usize,
    grid: &mut T,
) {
    let initial = BTreeSet::from([2, 1, 6, 7, 4, 5]);
    for x in 0..w {
        for y in 0..h {
            grid.with_tile(Location2D { x, y }, |t, _| {
                t.set_possible_states(initial.clone())
            });
        }
    }
    grid.get_lowest_entropy_position().expect(
        "get_lowest_entropy_position should've returned something with a newly initialized grid",
    );
    let mut expected = initial;

    let mut last = None;
    for position in [
        Location2D { x: 0, y: 0 },
        Location2D { x: w - 1, y: 0 },
        Location2D { x: 0, y: h - 1 },
        Location2D { x: w - 1, y: h - 1 },
    ] {
        let last_expected = expected.clone();
        expected.pop_last();
        grid.with_tile(position, |t, _| t.set_possible_states(expected.clone()));
        assert_eq!(
            grid.get_lowest_entropy_position().expect(
                "getting lowest entropy tile after a tile has been assigned a valid (>1) state"
            ),
            position,
            "grid.get_lowest_entropy_position should've updated after a tile was updated with a lower entropy"
        );
        if let Some(last_position) = last {
            grid.with_tile(last_position, |t, _| {
                t.set_possible_states(last_expected.clone())
            });
            assert_eq!(
                grid.get_lowest_entropy_position().expect(
                    "getting lowest entropy tile after a tile has been assigned a valid (>1) state"
                ),
                position,
                "grid.get_lowest_entropy_position should'nt've updated after a tile was updated with a higher entropy"
            );
        }
        last = Some(position);
    }
}

pub fn edges_2x2<
    T: GridInterface<NEIGHBOUR_COUNT_2D, TileState, Location2D, Direction2D, Tile>,
    F,
>(
    init: F,
) where
    F: Fn(RuleSet2D) -> T,
{
    const STATE_A: TileState = 0;
    const STATE_EDGE: TileState = 1;
    const STATE_B: TileState = 2;

    let generate_rules = |direction: Direction2D| {
        RuleSet2D::new(
            BTreeSet::from([STATE_A, STATE_EDGE, STATE_B]),
            HashSet::from([
                // identity, allow each state next to itself
                (STATE_A, Direction2D::LEFT, STATE_A),
                (STATE_A, Direction2D::UP, STATE_A),
                (STATE_EDGE, Direction2D::LEFT, STATE_EDGE),
                (STATE_EDGE, Direction2D::UP, STATE_EDGE),
                (STATE_B, Direction2D::LEFT, STATE_B),
                (STATE_B, Direction2D::UP, STATE_B),
                // allow A next to EDGE
                (STATE_A, Direction2D::UP, STATE_EDGE),
                (STATE_A, Direction2D::RIGHT, STATE_EDGE),
                (STATE_A, Direction2D::DOWN, STATE_EDGE),
                (STATE_A, Direction2D::LEFT, STATE_EDGE),
                // allow A next to B
                (STATE_A, Direction2D::UP, STATE_B),
                (STATE_A, Direction2D::RIGHT, STATE_B),
                (STATE_A, Direction2D::DOWN, STATE_B),
                (STATE_A, Direction2D::LEFT, STATE_B),
            ]),
            HashMap::new(),
            HashMap::new(),
            BTreeMap::from([(direction, STATE_EDGE)]),
        )
    };

    let expected_data = [
        (Direction2D::UP, [true, true, false, false]),
        (Direction2D::RIGHT, [false, true, false, true]),
        (Direction2D::DOWN, [false, false, true, true]),
        (Direction2D::LEFT, [true, false, true, false]),
    ];
    let tile_mappings = [
        Location2D { x: 0, y: 0 },
        Location2D { x: 1, y: 0 },
        Location2D { x: 0, y: 1 },
        Location2D { x: 1, y: 1 },
    ];

    for (direction, expected_tile_states) in expected_data {
        let grid = init(generate_rules(direction));
        for (i, expected_state) in expected_tile_states.iter().enumerate() {
            let tile_location = tile_mappings[i];
            let tile = grid.get_tile(tile_location).expect("failed to get tile");
            if *expected_state {
                assert!(tile.has_collapsed());
                assert_tile_state(&tile, STATE_EDGE);
            } else {
                // the collapse of the edge tiles should've removed B from possible states
                assert_eq!(tile.possible_states_ref().count(), 2);
            }
        }
    }
}
