use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    num::NonZeroUsize,
};

use crate::{
    tile::{TileState, interface::TileInterface},
    utils::space::{Direction, Location},
    wave_function_collapse::interface::{PropagateQueueEntry, TickResult, WaveFunctionCollapse},
};

use super::Backtracker;

#[derive(Debug)]
pub struct BacktrackerByGradualReset<TPosition: Location> {
    reset_count: HashMap<TPosition, NonZeroUsize>,
}

impl<TPosition: Location> BacktrackerByGradualReset<TPosition> {
    pub fn new() -> Self {
        Self {
            reset_count: HashMap::new(),
        }
    }
}

impl<
    const N: usize,
    TPosition: Location,
    TDirection: Direction<N>,
    T: TileInterface<TileState>,
    TGrid: WaveFunctionCollapse<N, TileState, TPosition, TDirection, T>,
> Backtracker<N, TileState, TPosition, TDirection, T, TGrid>
    for BacktrackerByGradualReset<TPosition>
{
    fn change_listener(&mut self, _change_location: TPosition, _new_states: T) {
        // no-op, we don't need this information
    }

    fn contradiction_handler(
        &mut self,
        grid: &mut TGrid,
        contradiction_location: TPosition,
    ) -> TickResult<TPosition> {
        let resets = self
            .reset_count
            .get(&contradiction_location)
            .map(|&u| u.checked_add(1).unwrap())
            .unwrap_or(NonZeroUsize::new(1).unwrap());
        self.reset_count.insert(contradiction_location, resets);

        let max_radius = usize::from(resets).pow(2);

        // gather an area of tiles around the contradiction to reset
        // we increase the area the more contradictions there have been at this location
        let mut locations_in_radius = BTreeSet::from([contradiction_location]);
        let mut locations_neighboring_radius = BTreeSet::new();
        let mut queue = VecDeque::from([(contradiction_location, 0)]);
        while let Some((current, distance)) = queue.pop_front() {
            println!("{current:?}, {distance}");
            if distance > max_radius {
                continue;
            }

            for (_, neighbour) in grid.get_neighbours(current) {
                if let Some(neighbour) = neighbour
                    && !locations_in_radius.contains(&neighbour)
                {
                    let target = if distance == max_radius {
                        &mut locations_in_radius
                    } else {
                        &mut locations_neighboring_radius
                    };
                    if target.insert(neighbour) && distance < (max_radius - 1) {
                        queue.push_back((neighbour, distance + 1));
                    }
                }
            }
        }
        println!("max {resets} -> {}", max_radius);
        println!("lir {:?}", locations_in_radius);
        println!("lir {:?}", locations_neighboring_radius);

        // small optimization: if we're about to reset all tiles, let's just reset the entire grid
        let tiles_in_grid = grid.get_dimensions().length();
        if locations_in_radius.len() == tiles_in_grid {
            grid.reset();
            return Ok(());
        }

        let rules_possible = grid.get_rules().possible.clone();

        // locations in the radius will be reset
        for location in &locations_in_radius {
            grid.with_tile(*location, |t, _| {
                t.set_possible_states(rules_possible.clone());
            });
        }

        let mut propagation_queue = VecDeque::new();
        for border_location in locations_neighboring_radius {
            // the bordering tiles will have their possible states recalculated
            grid.with_tile(border_location, |t, _| {
                t.set_possible_states(rules_possible.clone());
            });
            let neighbours = grid.get_neighbours(border_location);
            for (_direction, npos) in neighbours {
                if let Some(neighbour_position) = npos {
                    // we propagate changes into the reset area
                    if locations_in_radius.contains(&neighbour_position) {
                        propagation_queue.push_back(PropagateQueueEntry {
                            source: border_location,
                            target: neighbour_position,
                        });
                    }
                    // we recalculate the possible states of the bordering tile
                    propagation_queue.push_back(PropagateQueueEntry {
                        source: neighbour_position,
                        target: border_location,
                    });
                }
            }
        }

        grid.propagate(propagation_queue)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet, HashSet};

    use crate::{
        grid::{GridInterface, dynamic_2d::DynamicSizeGrid2D},
        rules::RuleSet2D,
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
    fn gradual_reset() {
        let target = Location2D { x: 0, y: 0 };
        let mut b = BacktrackerByGradualReset::new();

        // once, only the contradicting tile and it's neighbours should be reset
        let mut grid = gen_grid(target);
        b.contradiction_handler(&mut grid, target)
            .expect("contradiction should've resolved");

        for y in 0..2 {
            for x in 0..2 {
                let location = Location2D { x, y };
                let tile = grid.get_tile(location).unwrap();
                if location == target
                    || (x == target.x + 1 && y == target.y)
                    || (x == target.x && y == target.y + 1)
                {
                    assert_eq!(tile.possible_states().collect::<Vec<_>>(), vec![0, 1]);
                } else {
                    assert_eq!(tile.possible_states().collect::<Vec<_>>(), vec![0]);
                }
            }
        }

        // twice, the contradicting tile, it's neighbours and their neighbours should be reset
        // (all, in this case)
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
