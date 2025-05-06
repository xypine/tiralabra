use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::{
    tile::interface::TileInterface,
    utils::space::{Direction, Location},
    wave_function_collapse::interface::{TickResult, WaveFunctionCollapse},
};

use super::Backtracker;

#[derive(Serialize, Deserialize)]
pub struct BacktrackerByReset {}

impl<
    const N: usize,
    TState: Hash + Eq + Copy,
    TPosition: Location,
    TDirection: Direction<N>,
    T: TileInterface<TState>,
    TGrid: WaveFunctionCollapse<N, TState, TPosition, TDirection, T>,
> Backtracker<N, TState, TPosition, TDirection, T, TGrid> for BacktrackerByReset
{
    fn contradiction_handler(
        &mut self,
        grid: &mut TGrid,
        _contradiction_location: TPosition,
    ) -> TickResult<TPosition> {
        grid.reset();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

    use crate::{
        grid::{GridInterface, dynamic_2d::DynamicSizeGrid2D},
        rules::RuleSet2D,
        tile::TileState,
        utils::space::s2d::{Direction2D, Location2D},
    };

    use super::*;

    fn gen_grid(target: Location2D) -> DynamicSizeGrid2D {
        const STATE_A: TileState = 0;
        const STATE_B: TileState = 1;

        let rules = RuleSet2D::new(
            BTreeSet::from([STATE_A, STATE_B]),
            HashSet::from([
                (STATE_A, Direction2D::DOWN, STATE_A),
                (STATE_A, Direction2D::LEFT, STATE_A),
                (STATE_B, Direction2D::DOWN, STATE_B),
                (STATE_B, Direction2D::LEFT, STATE_B),
                (STATE_A, Direction2D::UP, STATE_B),
                (STATE_A, Direction2D::RIGHT, STATE_B),
                (STATE_A, Direction2D::DOWN, STATE_B),
                (STATE_A, Direction2D::LEFT, STATE_B),
            ]),
            HashMap::new(),
            HashMap::new(),
            BTreeMap::new(),
        );

        let mut grid = DynamicSizeGrid2D::new(2, 2, rules, 0);
        for y in 0..2 {
            for x in 0..2 {
                let location = Location2D { x, y };
                grid.with_tile(location, |t, _| {
                    if location == target {
                        t.set_possible_states([]);
                    } else {
                        t.set_possible_states([0]);
                    }
                });
            }
        }
        grid
    }

    #[test]
    fn whole_reset() {
        let target = Location2D { x: 0, y: 0 };
        let mut b = BacktrackerByReset {};

        // once, all tiles should be reset
        let mut grid = gen_grid(target);
        b.contradiction_handler(&mut grid, target)
            .expect("contradiction should've resolved");

        for y in 0..2 {
            for x in 0..2 {
                let location = Location2D { x, y };
                let tile = grid.get_tile(location).unwrap();
                assert_eq!(tile.possible_states().collect::<Vec<_>>(), vec![0, 1]);
            }
        }

        // twice, all tiles should be reset
        let mut grid = gen_grid(target);
        b.contradiction_handler(&mut grid, target)
            .expect("contradiction should've resolved");

        for y in 0..2 {
            for x in 0..2 {
                let location = Location2D { x, y };
                let tile = grid.get_tile(location).unwrap();
                assert_eq!(tile.possible_states().collect::<Vec<_>>(), vec![0, 1]);
            }
        }
    }
}
