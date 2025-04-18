use crate::{
    grid::{GridInterface, constant_2d::ConstantSizeGrid2D, tests::assert_tile_state},
    tile::interface::TileInterface,
    utils::space::Location2D,
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

    let mut grid = ConstantSizeGrid2D::<W, H>::new(rules);
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

    let mut grid = ConstantSizeGrid2D::<W, H>::new(rules);
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

    let mut grid = ConstantSizeGrid2D::<W, H>::new(rules);
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
    const W: usize = 3;
    const H: usize = 5;

    let rules = crate::rules::samples::flowers_singlepixel::rules();

    let mut grid = ConstantSizeGrid2D::<W, H>::new(rules);
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
            if y == H - 1 {
                assert_tile_state(&tile, STATE_GROUND);
            } else {
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

    let mut grid = ConstantSizeGrid2D::<W, H>::new(rules);
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
