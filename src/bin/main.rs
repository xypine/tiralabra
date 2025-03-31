use std::collections::{BTreeSet, HashSet};

use aaltofunktionromautus::{
    grid::ConstantSizeGrid2D,
    interface::{
        GridInterface, TileInterface, WaveFunctionCollapse, WaveFunctionCollapseInterruption,
    },
    rules::RuleSet,
    space::{Direction2D, Location2D},
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

fn main() {
    const W: usize = 12;
    const H: usize = 12;

    const STATE_BLACK: u64 = 0;
    const STATE_WHITE: u64 = 1;

    let possible = BTreeSet::from([STATE_BLACK, STATE_WHITE]);
    let allowed = HashSet::from([
        (STATE_BLACK, Direction2D::UP, STATE_WHITE),
        (STATE_BLACK, Direction2D::RIGHT, STATE_WHITE),
        (STATE_BLACK, Direction2D::DOWN, STATE_WHITE),
        (STATE_BLACK, Direction2D::LEFT, STATE_WHITE),
    ]);
    let rules = RuleSet::new(possible, allowed);

    let mut grid = ConstantSizeGrid2D::<W, H>::new(rules);
    debug_print(&grid);
    for _ in 0..((W * H) + 1) {
        let result = grid.tick();
        match result {
            Err(WaveFunctionCollapseInterruption::Finished) => break,
            Err(_) => result.unwrap(),
            Ok(_) => {}
        };
        // debug_print(&grid);
    }
    debug_print(&grid);
}
