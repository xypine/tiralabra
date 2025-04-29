pub mod reset;

use std::hash::Hash;

use crate::{
    tile::interface::TileInterface,
    utils::space::Direction,
    wave_function_collapse::interface::{TickResult, WaveFunctionCollapse},
};

pub trait Backtracker<
    const NEIGHBOURS_PER_TILE: usize,
    TState: Hash + Eq + Copy,
    TPosition,
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
}
