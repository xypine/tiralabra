//! Generic interfaces that the WFC algorithm is based upon
//!
//! Quite useful, as they allow it to work with multiple backing Grid or Tile implementations

use std::{collections::VecDeque, hash::Hash};

use serde::Serialize;

use crate::{
    backtracking::Backtracker, grid::GridInterface, tile::interface::TileInterface,
    utils::space::Direction,
};

/// Used when the algorithm has to return early for some reason
#[derive(Debug, Clone, Copy, thiserror::Error, Serialize)]
pub enum WaveFunctionCollapseInterruption<TPosition> {
    /// All tiles in the grid have been successfully collapsed
    Finished,
    /// A tile has lost all of it's possible states,
    /// the algorithm cannot continue without backtracking
    Contradiction(TPosition),
    /// The algorithm did not complete in the allocated iterations
    MaxIterationsReached,
}

/// Used for tracking which tiles will need to be visisted during propagation
#[derive(Debug)]
pub struct PropagateQueueEntry<TPosition> {
    /// Which neighbour was updated to prompt the need for propagation
    pub source: TPosition,
    /// Which tile's states will be rechecked
    pub target: TPosition,
}

pub type TickResult<TPosition> = Result<(), WaveFunctionCollapseInterruption<TPosition>>;

pub trait WaveFunctionCollapse<
    const NEIGHBOURS_PER_TILE: usize,
    TState: Hash + Eq + Copy,
    TPosition,
    TDirection: Direction<{ NEIGHBOURS_PER_TILE }>,
    T: TileInterface<TState, TPosition>,
>: GridInterface<NEIGHBOURS_PER_TILE, TState, TPosition, TDirection, T>
{
    /// Forces a tile at the given position into one of it's possible states.
    /// If no value is provided, on is picked randomly.
    ///
    /// If no possible states remain, a contradiction interruption is returned.
    fn collapse(
        &mut self,
        position: TPosition,
        value: Option<TState>,
    ) -> Result<(), WaveFunctionCollapseInterruption<TPosition>>;

    /// Propagates changes to a tile to it's neighbours, updating their possible states.
    /// If a neighbour is modified, we then propagate to it's neighbours and so on
    fn propagate(
        &mut self,
        queue: VecDeque<PropagateQueueEntry<TPosition>>,
    ) -> TickResult<TPosition>;

    fn tick(&mut self) -> TickResult<TPosition>;

    /// Runs the algorithm until all tiles have been collapsed, a contradiction occurs or a maximum
    /// amount of iterations is reached
    // automatically implemented for all types that implement WaveFunctionCollapse
    fn run<B: Backtracker<NEIGHBOURS_PER_TILE, TState, TPosition, TDirection, T, Self>>(
        &mut self,
        max_iterations: usize,
        mut backtracker: Option<B>,
    ) -> TickResult<TPosition> {
        for _ in 0..max_iterations {
            let result = self.tick();
            match result {
                Ok(()) => continue,
                Err(WaveFunctionCollapseInterruption::Contradiction(e)) => {
                    if let Some(handler) = backtracker.as_mut() {
                        handler.contradiction_handler(self, e)?;
                    } else {
                        return Err(WaveFunctionCollapseInterruption::Contradiction(e));
                    }
                }
                Err(e) => return Err(e),
            }
        }

        // some tiles were left uncollapsed in the given time
        Err(WaveFunctionCollapseInterruption::MaxIterationsReached)
    }
}
