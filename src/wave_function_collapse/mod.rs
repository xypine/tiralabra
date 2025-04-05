//! Main implementation of the algorithm

#[cfg(test)]
mod e2e_tests;

use std::collections::VecDeque;

use crate::{
    interface::{
        GridInterface, PropagateQueueEntry, TileInterface, WaveFunctionCollapse,
        WaveFunctionCollapseInterruption,
    },
    tile::{Tile, TileState},
    utils::space::{Direction2D, Location2D, NEIGHBOUR_COUNT_2D},
};

// Implements the Wave Function Collapse algorithm for any struct that implements `GridInterface`
// See the trait for further documentation about the methods
impl<T: GridInterface<NEIGHBOUR_COUNT_2D, TileState, Location2D, Direction2D, Tile>>
    WaveFunctionCollapse<Location2D, TileState> for T
{
    fn collapse(
        &mut self,
        position: Location2D,
        value: Option<TileState>,
    ) -> Result<(), crate::interface::WaveFunctionCollapseInterruption<Location2D>> {
        self.with_tile(position, |tile| tile.collapse(value))
            .flatten()
            .ok_or(WaveFunctionCollapseInterruption::Contradiction(position))?;

        let neighbours = self.get_neighbours(position);
        for (_direction, npos) in neighbours {
            if let Some(neighbour_position) = npos {
                self.propagate(VecDeque::from([PropagateQueueEntry {
                    source: position,
                    target: neighbour_position,
                }]))?;
            }
        }

        Ok(())
    }

    fn propagate(
        &mut self,
        mut queue: VecDeque<crate::interface::PropagateQueueEntry<Location2D>>,
    ) -> crate::interface::TickResult<Location2D> {
        while let Some(queue_entry) = queue.pop_front() {
            let delta = queue_entry
                .target
                .delta(queue_entry.source)
                .expect("converting propagate location to delta");
            let direction =
                Direction2D::try_from(delta).expect("converting propagate delta to direction");
            // println!("{queue_entry:?} {direction:?}");
            let source = self
                .get_tile(queue_entry.source)
                .expect("getting propagation source");
            let rules = self.get_rules().clone();
            let was_modified = self
                .with_tile(queue_entry.target, |target| {
                    if target.has_collapsed() {
                        return false;
                    }
                    let unmodified_length = target.possible_states_ref().count();
                    let checked_states = rules.check(target, &source, direction);
                    let modified_length = checked_states.len();
                    let was_modified = unmodified_length != modified_length;
                    if was_modified {
                        target.set_possible_states(checked_states);
                    }
                    was_modified
                })
                .expect("updating tile during propagation");
            if was_modified {
                let neighbours = self.get_neighbours(queue_entry.target);
                for (_direction, npos) in neighbours {
                    if let Some(neighbour_position) = npos {
                        queue.push_back(PropagateQueueEntry {
                            source: queue_entry.target,
                            target: neighbour_position,
                        });
                    }
                }
            }
        }
        Ok(())
    }

    fn tick(&mut self) -> crate::interface::TickResult<Location2D> {
        let lowest_entropy = self
            .get_lowest_entropy_position()
            .ok_or(WaveFunctionCollapseInterruption::Finished::<Location2D>)?;

        self.collapse(lowest_entropy, None)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashSet};

    use crate::{grid::constant_2d::ConstantSizeGrid2D, rules::RuleSet};

    use super::*;

    #[test]
    fn find_lowest_entropy_sanity() {
        const W: usize = 2;
        const H: usize = 2;
        let rules = RuleSet::new(BTreeSet::from([1, 2, 3]), HashSet::from([]));
        let mut grid: ConstantSizeGrid2D<W, H> = ConstantSizeGrid2D::new(rules);

        let lowest_entropy_location = Location2D { x: 0, y: 1 };
        assert_eq!(
            grid.get_tile(lowest_entropy_location)
                .unwrap()
                .possible_states()
                .count(),
            3
        );

        grid.with_tile(lowest_entropy_location, |t| {
            t.set_possible_states(BTreeSet::from([1, 2]))
        });

        assert_eq!(
            grid.get_tile(lowest_entropy_location)
                .unwrap()
                .possible_states()
                .count(),
            2
        );

        let implementation = grid.get_lowest_entropy_position().unwrap();
        assert_eq!(lowest_entropy_location, implementation);
    }
}
