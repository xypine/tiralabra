//! Generic interfaces that the WFC algorithm is based upon
//!
//! Quite useful, as they allow it to work with multiple backing Grid or Tile implementations

use std::collections::VecDeque;

use serde::Serialize;

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

pub trait WaveFunctionCollapse<TPosition, TValue> {
    /// Forces a tile at the given position into one of it's possible states.
    /// If no value is provided, on is picked randomly.
    ///
    /// If no possible states remain, a contradiction interruption is returned.
    fn collapse(
        &mut self,
        position: TPosition,
        value: Option<TValue>,
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
    fn run(&mut self, max_iterations: usize) -> TickResult<TPosition> {
        for _ in 0..max_iterations {
            self.tick()?;
        }

        // some tiles were left uncollapsed in the given time
        Err(WaveFunctionCollapseInterruption::MaxIterationsReached)
    }
}
