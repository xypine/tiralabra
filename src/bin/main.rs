use std::collections::{BTreeSet, HashSet};

use aaltofunktionromautus::{
    grid::{constant_2d::ConstantSizeGrid2D, dynamic_2d::DynamicSizeGrid2D},
    interface::{
        GridInterface, TileInterface, WaveFunctionCollapse, WaveFunctionCollapseInterruption,
    },
    rules::RuleSet,
    tile::{Tile, TileState},
    utils::space::{Direction2D, Location2D, NEIGHBOUR_COUNT_2D},
};

fn debug_print<const W: usize, const H: usize>(
    grid: &impl GridInterface<NEIGHBOUR_COUNT_2D, TileState, Location2D, Direction2D, Tile>,
) {
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

fn main() {
    const W: usize = 30;
    const H: usize = 30;

    const STATE_DEEP_SEA: u64 = 0;
    const STATE_SEA: u64 = 1;
    const STATE_SHORE: u64 = 2;
    const STATE_LAND: u64 = 3;
    const STATE_FOREST: u64 = 4;

    let possible = BTreeSet::from([
        STATE_DEEP_SEA,
        STATE_SEA,
        STATE_SHORE,
        STATE_LAND,
        STATE_FOREST,
    ]);
    let allowed = HashSet::from([
        // identity rules, allow x next to x
        (STATE_DEEP_SEA, Direction2D::UP, STATE_DEEP_SEA),
        (STATE_DEEP_SEA, Direction2D::RIGHT, STATE_DEEP_SEA),
        (STATE_DEEP_SEA, Direction2D::DOWN, STATE_DEEP_SEA),
        (STATE_DEEP_SEA, Direction2D::LEFT, STATE_DEEP_SEA),
        (STATE_SEA, Direction2D::UP, STATE_SEA),
        (STATE_SEA, Direction2D::RIGHT, STATE_SEA),
        (STATE_SEA, Direction2D::DOWN, STATE_SEA),
        (STATE_SEA, Direction2D::LEFT, STATE_SEA),
        (STATE_SHORE, Direction2D::UP, STATE_SHORE),
        (STATE_SHORE, Direction2D::RIGHT, STATE_SHORE),
        (STATE_SHORE, Direction2D::DOWN, STATE_SHORE),
        (STATE_SHORE, Direction2D::LEFT, STATE_SHORE),
        (STATE_LAND, Direction2D::UP, STATE_LAND),
        (STATE_LAND, Direction2D::RIGHT, STATE_LAND),
        (STATE_LAND, Direction2D::DOWN, STATE_LAND),
        (STATE_LAND, Direction2D::LEFT, STATE_LAND),
        (STATE_FOREST, Direction2D::UP, STATE_FOREST),
        (STATE_FOREST, Direction2D::RIGHT, STATE_FOREST),
        (STATE_FOREST, Direction2D::DOWN, STATE_FOREST),
        (STATE_FOREST, Direction2D::LEFT, STATE_FOREST),
        // adjacency rules, allow DEEP_SEA -> SEA -> SHORE -> LAND -> FOREST
        (STATE_DEEP_SEA, Direction2D::UP, STATE_SEA),
        (STATE_DEEP_SEA, Direction2D::RIGHT, STATE_SEA),
        (STATE_DEEP_SEA, Direction2D::DOWN, STATE_SEA),
        (STATE_DEEP_SEA, Direction2D::LEFT, STATE_SEA),
        (STATE_SEA, Direction2D::UP, STATE_SHORE),
        (STATE_SEA, Direction2D::RIGHT, STATE_SHORE),
        (STATE_SEA, Direction2D::DOWN, STATE_SHORE),
        (STATE_SEA, Direction2D::LEFT, STATE_SHORE),
        (STATE_SHORE, Direction2D::UP, STATE_LAND),
        (STATE_SHORE, Direction2D::RIGHT, STATE_LAND),
        (STATE_SHORE, Direction2D::DOWN, STATE_LAND),
        (STATE_SHORE, Direction2D::LEFT, STATE_LAND),
        (STATE_LAND, Direction2D::UP, STATE_FOREST),
        (STATE_LAND, Direction2D::RIGHT, STATE_FOREST),
        (STATE_LAND, Direction2D::DOWN, STATE_FOREST),
        (STATE_LAND, Direction2D::LEFT, STATE_FOREST),
    ]);
    let rules = RuleSet::new(possible, allowed);

    let mut grid = DynamicSizeGrid2D::new(W, H, rules);
    for i in 0..((W * H) + 1) {
        println!("iteration {i}");
        debug_print::<W, H>(&grid);
        let result = grid.tick();
        match result {
            Err(WaveFunctionCollapseInterruption::Finished) => break,
            Err(_) => result.unwrap(),
            Ok(_) => {}
        };
    }
    println!("w * h = {}", W * H);
    debug_print::<W, H>(&grid);
}
