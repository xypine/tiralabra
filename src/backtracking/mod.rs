pub mod gradual_reset;
pub mod reset;

use std::hash::Hash;

use gradual_reset::BacktrackerByGradualReset;
use reset::BacktrackerByReset;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::{
    tile::{TileState, interface::TileInterface},
    utils::space::{Direction, Location, s2d::Location2D},
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

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Backtracker2D {
    Reset(BacktrackerByReset),
    GradualReset(BacktrackerByGradualReset<Location2D>),
}

impl<
    const N: usize,
    TDirection: Direction<N>,
    T: TileInterface<TileState>,
    TGrid: WaveFunctionCollapse<N, TileState, Location2D, TDirection, T>,
> Backtracker<N, TileState, Location2D, TDirection, T, TGrid> for Backtracker2D
{
    fn contradiction_handler(
        &mut self,
        grid: &mut TGrid,
        contradiction_location: Location2D,
    ) -> TickResult<Location2D> {
        match self {
            Backtracker2D::Reset(backtracker_by_reset) => {
                backtracker_by_reset.contradiction_handler(grid, contradiction_location)
            }
            Backtracker2D::GradualReset(backtracker_by_gradual_reset) => {
                backtracker_by_gradual_reset.contradiction_handler(grid, contradiction_location)
            }
        }
    }
}
