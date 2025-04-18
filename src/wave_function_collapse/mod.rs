//! Main implementation of the algorithm

#[cfg(test)]
mod e2e_tests;
pub mod interface;

use std::collections::{BTreeSet, VecDeque};

use interface::{
    PropagateQueueEntry, TickResult, WaveFunctionCollapse, WaveFunctionCollapseInterruption,
};

use crate::{
    grid::GridInterface,
    tile::{
        interface::TileInterface,
        {Tile, TileState},
    },
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
    ) -> Result<(), WaveFunctionCollapseInterruption<Location2D>> {
        self.with_tile(position, |tile| tile.collapse(value))
            .flatten()
            .ok_or(WaveFunctionCollapseInterruption::Contradiction(position))?;

        let neighbours = self.get_neighbours(position);
        let initial_queue =
            VecDeque::from_iter(neighbours.into_iter().flat_map(|(_direction, npos)| {
                npos.map(|neighbour_position| PropagateQueueEntry {
                    source: position,
                    target: neighbour_position,
                })
            }));
        self.propagate(initial_queue)?;

        Ok(())
    }

    fn propagate(
        &mut self,
        mut queue: VecDeque<PropagateQueueEntry<Location2D>>,
    ) -> TickResult<Location2D> {
        while let Some(queue_entry) = queue.pop_front() {
            let delta = queue_entry
                .target
                .delta(queue_entry.source)
                .expect("converting propagate location to delta");
            let direction =
                Direction2D::try_from(delta).expect("converting propagate delta to direction");
            let source = self
                .get_tile(queue_entry.source)
                .expect("getting propagation source");
            let rules = self.get_rules().clone();
            let should_propagate = self
                .with_tile(queue_entry.target, |target| {
                    let old_states: BTreeSet<_> = target.possible_states().collect();
                    let checked_states = rules.check(target, &source, direction);
                    if checked_states.is_empty() {
                        return Err(WaveFunctionCollapseInterruption::Contradiction(
                            queue_entry.target,
                        ));
                    }
                    let was_modified = old_states != checked_states;
                    if was_modified {
                        target.set_possible_states(checked_states);
                    }
                    Ok(was_modified)
                })
                .expect("updating tile during propagation")?;
            if should_propagate {
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

    fn tick(&mut self) -> TickResult<Location2D> {
        let lowest_entropy = self
            .get_lowest_entropy_position()
            .ok_or(WaveFunctionCollapseInterruption::Finished::<Location2D>)?;

        self.collapse(lowest_entropy, None)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashMap, HashSet};

    use crate::{
        grid::constant_2d::ConstantSizeGrid2D,
        rules::{RuleSet, RuleSet2D},
    };

    use super::*;

    #[test]
    fn find_lowest_entropy_sanity() {
        const W: usize = 2;
        const H: usize = 2;
        let rules = RuleSet::new(BTreeSet::from([1, 2, 3]), HashSet::from([]), HashMap::new());
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

    #[test]
    fn propagation_race_condition_cw() {
        // With an incorrect implementation of the algorithm, we might end up in a situation where
        // we collapse a cell into an invalid state, as a previous collapse hasn't propagated this
        // far _yet_.

        const STATE_ONE: TileState = 1;
        const STATE_TWO: TileState = 2;
        const STATE_THREE: TileState = 3;
        const STATE_FOUR: TileState = 4;
        let rules = RuleSet2D::new(
            BTreeSet::from([STATE_ONE, STATE_TWO, STATE_THREE, STATE_FOUR]),
            HashSet::from([
                (STATE_ONE, Direction2D::RIGHT, STATE_TWO),
                (STATE_TWO, Direction2D::DOWN, STATE_THREE),
                (STATE_THREE, Direction2D::LEFT, STATE_FOUR),
            ]),
            HashMap::new(),
        );

        const W: usize = 2;
        const H: usize = 2;
        let mut grid: ConstantSizeGrid2D<W, H> = ConstantSizeGrid2D::new(rules);
        let result = grid.collapse(Location2D { x: 0, y: 0 }, Some(STATE_ONE));

        assert!(
            matches!(result, Err(WaveFunctionCollapseInterruption::Contradiction(location)) if location == Location2D{x: 0, y: 1})
        );
    }

    #[test]
    fn propagation_race_condition_ccw() {
        // see propagation_race_condition_a_cw

        const STATE_ONE: TileState = 1;
        const STATE_TWO: TileState = 2;
        const STATE_THREE: TileState = 3;
        const STATE_FOUR: TileState = 4;
        let rules = RuleSet2D::new(
            BTreeSet::from([STATE_ONE, STATE_TWO, STATE_THREE, STATE_FOUR]),
            HashSet::from([
                (STATE_ONE, Direction2D::LEFT, STATE_TWO),
                (STATE_TWO, Direction2D::UP, STATE_THREE),
                (STATE_THREE, Direction2D::RIGHT, STATE_FOUR),
            ]),
            HashMap::new(),
        );

        const W: usize = 2;
        const H: usize = 2;
        let mut grid: ConstantSizeGrid2D<W, H> = ConstantSizeGrid2D::new(rules);
        let result = grid.collapse(Location2D { x: 1, y: 1 }, Some(STATE_ONE));

        assert!(
            matches!(result, Err(WaveFunctionCollapseInterruption::Contradiction(location)) if location == Location2D{x: 1, y: 0})
        );
    }
}
