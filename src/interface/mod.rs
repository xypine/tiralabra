pub mod wasm;

use rand::Rng;
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

use crate::{rules::RuleSet, utils::entropy::Entropy};

pub trait TileInterface<State, TCoords> {
    /// Returns an iterator over the possible states of the tile.
    /// No data is copied so the usage of this function should be quite efficient
    fn possible_states_ref<'a>(&'a self) -> impl Iterator<Item = &'a State>
    where
        State: 'a;

    /// Returns an iterator over copies of possible states of the tile.
    fn possible_states(&self) -> impl Iterator<Item = State>;

    // this is a naive implementation that only relies on the trait.
    // As such, it is probably quite inefficient and should be overridden on the implementing
    // struct
    fn has_collapsed(&self) -> bool {
        self.possible_states().count() == 1
    }

    /// "Entropy" is used to pick the best tile to collapse
    /// Basically tiles with few remaining possible states should have low entropy
    fn calculate_entropy(&self) -> Option<Entropy> {
        if self.has_collapsed() {
            return None;
        }
        // TODO: Replace with the actual entropy calculation
        let possible = self.possible_states().count();
        let entropy = possible as f64;
        let mut rng = rand::rng();
        let random = rng.random_range(0.0..0.2);
        Some(Entropy(entropy + random))
    }

    /// Forces the tile into a single state.
    /// If no value is provided, one is chosen from the currently available states.
    /// If no available states exist, `None` is returned
    fn collapse(&mut self, value: Option<State>) -> Option<State>;
}

pub trait Location<const DIMENSIONS: usize> {}
pub trait Direction<const COUNT: usize>: Hash + Eq {
    fn mirror(self) -> Self;
}

/// A grid contains cells
/// the interface is dimension agnostic
pub trait GridInterface<
    const NEIGHBOURS_PER_TILE: usize,
    TState,
    TPosition,
    TDirection: Direction<{ NEIGHBOURS_PER_TILE }>,
    T: TileInterface<TState, TPosition>,
>
{
    fn image(&self) -> HashMap<TPosition, T>;
    fn get_tile(&self, location: TPosition) -> Option<T>;
    fn get_neighbours(
        &self,
        location: TPosition,
    ) -> [(TDirection, Option<TPosition>); NEIGHBOURS_PER_TILE];
    fn get_neighbour_tiles(
        &self,
        location: TPosition,
    ) -> [(TDirection, Option<T>); NEIGHBOURS_PER_TILE];

    fn get_lowest_entropy_position(&mut self) -> Option<TPosition>;

    /// Fetches the tile at the given location and gives you mutable access to it
    fn with_tile<R, F: Fn(&mut T) -> R>(&mut self, location: TPosition, f: F) -> Option<R>;

    fn get_rules(&self) -> RuleSet<NEIGHBOURS_PER_TILE, TDirection>;
}

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
