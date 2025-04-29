use std::hash::Hash;

use crate::{
    tile::interface::TileInterface,
    utils::space::Direction,
    wave_function_collapse::interface::{TickResult, WaveFunctionCollapse},
};

use super::Backtracker;

pub struct BacktrackerByReset {}

impl<
    const N: usize,
    TState: Hash + Eq + Copy,
    TPosition: Copy,
    TDirection: Direction<N>,
    T: TileInterface<TState>,
    TGrid: WaveFunctionCollapse<N, TState, TPosition, TDirection, T>,
> Backtracker<N, TState, TPosition, TDirection, T, TGrid> for BacktrackerByReset
{
    fn change_listener(&mut self, _change_location: TPosition, _new_states: T) {
        // no-op, we don't need this information
    }

    fn contradiction_handler(
        &mut self,
        grid: &mut TGrid,
        _contradiction_location: TPosition,
    ) -> TickResult<TPosition> {
        grid.reset();
        Ok(())
    }
}
