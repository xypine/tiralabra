pub mod gradual_reset;
pub mod reset;

use std::hash::Hash;

use crate::{
    tile::interface::TileInterface,
    utils::space::{Direction, Location},
    wave_function_collapse::interface::{
        TickResult, WaveFunctionCollapse, WaveFunctionCollapseInterruption,
    },
};

pub trait Backtracker<
    const NEIGHBOURS_PER_TILE: usize,
    TState: Hash + Eq + Copy,
    TPosition: Location,
    TDirection: Direction<{ NEIGHBOURS_PER_TILE }>,
    T: TileInterface<TState>,
    TGrid: WaveFunctionCollapse<NEIGHBOURS_PER_TILE, TState, TPosition, TDirection, T>,
>
{
    /// Returns a closure that can be called on tile change.
    fn change_listener(&mut self, change_location: TPosition, new_states: T);

    /// Returns a closure that handles contradictions.
    fn contradiction_handler(
        &mut self,
        grid: &mut TGrid,
        contradiction_location: TPosition,
    ) -> TickResult<TPosition>;

    fn contradiction_handler_recursive(
        &mut self,
        grid: &mut TGrid,
        contradiction_location: TPosition,
        max_tries: usize,
    ) -> TickResult<TPosition> {
        let mut result = self.contradiction_handler(grid, contradiction_location);
        let mut tries = 1;
        loop {
            if tries > max_tries {
                return result;
            }
            match result {
                Ok(_) => return Ok(()),
                Err(WaveFunctionCollapseInterruption::Finished) => return Ok(()),
                Err(WaveFunctionCollapseInterruption::Contradiction(p)) => {
                    result = self.contradiction_handler(grid, p)
                }
                Err(WaveFunctionCollapseInterruption::MaxIterationsReached) => {
                    return Err(WaveFunctionCollapseInterruption::MaxIterationsReached);
                }
            }
            tries += 1;
        }
    }
}
