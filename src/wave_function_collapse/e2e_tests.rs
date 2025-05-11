use crate::{
    backtracking::{gradual_reset::BacktrackerByGradualReset, reset::BacktrackerByReset},
    grid::{GridInterface, constant_2d::ConstantSizeGrid2D, tests::assert_tile_state},
    tile::interface::TileInterface,
    utils::space::s2d::Location2D,
    wave_function_collapse::interface::{WaveFunctionCollapse, WaveFunctionCollapseInterruption},
};

fn debug_print<const W: usize, const H: usize>(grid: &ConstantSizeGrid2D<W, H>) {
    for y in 0..H {
        for x in 0..W {
            let tile = grid.get_tile(Location2D { x, y }).unwrap();
            let mut repr = format!("?{}", tile.possible_states().count());
            if tile.has_collapsed() {
                repr = tile.possible_states().next().unwrap().to_string();
            }
            print!("{repr: <4}")
        }
        println!()
    }
    println!()
}

#[test]
fn checkers_a() {
    use crate::rules::samples::checkers::{STATE_BLACK, STATE_WHITE};
    const W: usize = 2;
    const H: usize = 2;

    let rules = crate::rules::samples::checkers::rules();

    let mut grid = ConstantSizeGrid2D::<W, H>::new(rules, 0);
    let result = grid.collapse(Location2D { x: 0, y: 0 }, Some(STATE_BLACK));
    match result {
        Err(WaveFunctionCollapseInterruption::Finished) => (),
        Err(_) => result.unwrap(),
        Ok(_) => {}
    };
    debug_print(&grid);

    assert_tile_state(
        &grid.get_tile(Location2D { x: 0, y: 0 }).unwrap(),
        STATE_BLACK,
    );
    assert_tile_state(
        &grid.get_tile(Location2D { x: 1, y: 0 }).unwrap(),
        STATE_WHITE,
    );
    assert_tile_state(
        &grid.get_tile(Location2D { x: 0, y: 1 }).unwrap(),
        STATE_WHITE,
    );
    assert_tile_state(
        &grid.get_tile(Location2D { x: 1, y: 1 }).unwrap(),
        STATE_BLACK,
    );
}
#[test]
fn checkers_b() {
    use crate::rules::samples::checkers::{STATE_BLACK, STATE_WHITE};
    const W: usize = 2;
    const H: usize = 2;

    let rules = crate::rules::samples::checkers::rules();

    let mut grid = ConstantSizeGrid2D::<W, H>::new(rules, 0);
    let result = grid.collapse(Location2D { x: 0, y: 0 }, Some(STATE_WHITE));
    match result {
        Err(WaveFunctionCollapseInterruption::Finished) => (),
        Err(_) => result.unwrap(),
        Ok(_) => {}
    };
    debug_print(&grid);

    assert_tile_state(
        &grid.get_tile(Location2D { x: 0, y: 0 }).unwrap(),
        STATE_WHITE,
    );
    assert_tile_state(
        &grid.get_tile(Location2D { x: 1, y: 0 }).unwrap(),
        STATE_BLACK,
    );
    assert_tile_state(
        &grid.get_tile(Location2D { x: 0, y: 1 }).unwrap(),
        STATE_BLACK,
    );
    assert_tile_state(
        &grid.get_tile(Location2D { x: 1, y: 1 }).unwrap(),
        STATE_WHITE,
    );
}

#[test]
fn stripes() {
    use crate::rules::samples::stripes::{STATE_MIDDLE, STATE_ONE, STATE_TWO};
    const W: usize = 2;
    const H: usize = 2;

    let rules = crate::rules::samples::stripes::rules();

    let mut grid = ConstantSizeGrid2D::<W, H>::new(rules, 0);
    let result = grid.collapse(Location2D { x: 0, y: 0 }, Some(STATE_ONE));
    match result {
        Err(WaveFunctionCollapseInterruption::Finished) => (),
        Err(_) => result.unwrap(),
        Ok(_) => {}
    };
    debug_print(&grid);

    assert_tile_state(
        &grid.get_tile(Location2D { x: 0, y: 0 }).unwrap(),
        STATE_ONE,
    );
    assert_tile_state(
        &grid.get_tile(Location2D { x: 1, y: 0 }).unwrap(),
        STATE_MIDDLE,
    );
    assert_tile_state(
        &grid.get_tile(Location2D { x: 0, y: 1 }).unwrap(),
        STATE_MIDDLE,
    );
    assert_tile_state(
        &grid.get_tile(Location2D { x: 1, y: 1 }).unwrap(),
        STATE_TWO,
    );
}

#[test]
fn flowers_a() {
    use crate::rules::samples::flowers_singlepixel::STATE_GROUND;
    const W: usize = 9;
    const H: usize = 9;

    let rules = crate::rules::samples::flowers_singlepixel::rules();

    let mut grid = ConstantSizeGrid2D::<W, H>::new(rules, 0);
    let result = grid.collapse(Location2D { x: 1, y: H - 1 }, Some(STATE_GROUND));
    match result {
        Err(WaveFunctionCollapseInterruption::Finished) => panic!(
            "Collapsing a ground tile shouldn't have finished a 3x3 grid using the flowers ruleset"
        ),
        Err(_) => result.unwrap(),
        Ok(_) => {}
    };
    debug_print(&grid);

    for x in 0..W {
        for y in 0..H {
            let tile = grid.get_tile(Location2D { x, y }).unwrap();
            if y < 2 || x < 2 || x > W - 3 {
                assert!(tile.has_collapsed(), "all edge tiles should've collapsed")
            } else if y == H - 1 {
                assert_tile_state(&tile, STATE_GROUND);
            } else {
                if tile.has_collapsed() {
                    println!("{x}, {y} was collapsed!");
                }
                assert!(
                    !tile.has_collapsed(),
                    "only ground tiles should've collapsed"
                );
            }
        }
    }
}

#[test]
fn terrain() {
    const W: usize = 9;
    const H: usize = 9;

    let rules = crate::rules::samples::terrain::rules();

    let mut grid = ConstantSizeGrid2D::<W, H>::new(rules, 0);
    debug_print(&grid);
    for _ in 0..((W * H) + 1) {
        let result = grid.tick();
        match result {
            Err(WaveFunctionCollapseInterruption::Finished) => break,
            Err(_) => result.unwrap(),
            Ok(_) => {}
        };
        debug_print(&grid);
    }
}

#[test]
#[ignore = "slow"]
fn flowers_blatant_rule_violations() {
    use rayon::iter::IntoParallelIterator;
    use rayon::iter::ParallelIterator;
    use std::collections::BTreeSet;

    const W: usize = 15;
    const H: usize = 15;

    let rules = crate::rules::samples::flowers_singlepixel::rules();

    (0..1000).into_par_iter().for_each(|seed| {
        let mut grid = ConstantSizeGrid2D::<W, H>::new(rules.clone(), seed);
        let result = grid.run(500, Some(BacktrackerByReset {}));
        match result {
            Err(WaveFunctionCollapseInterruption::Finished) => {}
            Err(_) => result.unwrap(),
            Ok(_) => panic!("Grid should've finished"),
        };
        println!("seed {seed}");
        debug_print(&grid);

        for x in 0..W {
            for y in 0..H {
                let location = Location2D { x, y };
                let mut tile = grid.get_tile(location).unwrap().clone();
                let old_states: BTreeSet<_> = tile.possible_states().collect();
                for (neighbour_dir, neighbour) in grid.get_neighbour_tiles(location) {
                    if let Some(neighbour) = neighbour {
                        let checked_states = rules.check(&tile, neighbour, neighbour_dir);
                        tile.set_possible_states(checked_states);
                    }
                }
                let checked_states: BTreeSet<_> = tile.possible_states().collect();
                assert_eq!(old_states, checked_states);
            }
        }
    });
}

#[test]
#[ignore = "slow"]
fn flowers_blatant_rule_violations_gradual_reset() {
    use rayon::iter::IntoParallelIterator;
    use rayon::iter::ParallelIterator;
    use std::collections::BTreeSet;

    const W: usize = 15;
    const H: usize = 15;

    let rules = crate::rules::samples::flowers_singlepixel::rules();

    (0..1000).into_par_iter().for_each(|seed| {
        let mut grid = ConstantSizeGrid2D::<W, H>::new(rules.clone(), seed);
        let result = grid.run(500, Some(BacktrackerByGradualReset::new(1)));
        match result {
            Err(WaveFunctionCollapseInterruption::Finished) => {}
            Err(_) => result.unwrap(),
            Ok(_) => panic!("Grid should've finished"),
        };
        println!("seed {seed}");
        debug_print(&grid);

        for x in 0..W {
            for y in 0..H {
                let location = Location2D { x, y };
                let mut tile = grid.get_tile(location).unwrap().clone();
                let old_states: BTreeSet<_> = tile.possible_states().collect();
                for (neighbour_dir, neighbour) in grid.get_neighbour_tiles(location) {
                    if let Some(neighbour) = neighbour {
                        let checked_states = rules.check(&tile, neighbour, neighbour_dir);
                        tile.set_possible_states(checked_states);
                    }
                }
                let checked_states: BTreeSet<_> = tile.possible_states().collect();
                assert_eq!(old_states, checked_states);
            }
        }
    });
}
