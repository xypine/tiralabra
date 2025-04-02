pub mod wasm;

use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

use crate::{rules::RuleSet, utils::entropy::Entropy};

pub trait TileInterface<State, TCoords> {
    fn possible_states(&self) -> impl Iterator<Item = State>;
    fn possible_states_ref<'a>(&'a self) -> impl Iterator<Item = &'a State>
    where
        State: 'a;

    // this is a naive implementation that only relies on the trait
    // as such, it is probably quite inefficient
    fn has_collapsed(&self) -> bool {
        self.possible_states().count() == 1
    }
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

    fn collapse(&mut self) -> Option<State>;
}

pub trait Location<const DIMENSIONS: usize> {}
pub trait Direction<const COUNT: usize>: Hash + Eq {
    fn mirror(self) -> Self;
}

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

use rand::Rng;
use serde::Serialize;
#[derive(Debug, Clone, Copy, thiserror::Error, Serialize)]
pub enum WaveFunctionCollapseInterruption<TPosition> {
    Finished,
    Contradiction(TPosition),
    MaxIterationsReached,
}

#[derive(Debug)]
pub struct PropagateQueueEntry<TPosition> {
    pub source: TPosition,
    pub target: TPosition,
}

pub type TickResult<TPosition> = Result<(), WaveFunctionCollapseInterruption<TPosition>>;
pub trait WaveFunctionCollapse<TPosition> {
    fn find_lowest_entropy(&mut self) -> Option<TPosition>;
    fn collapse(
        &mut self,
        position: TPosition,
    ) -> Result<(), WaveFunctionCollapseInterruption<TPosition>>;
    // breadth first
    fn propagate(
        &mut self,
        queue: VecDeque<PropagateQueueEntry<TPosition>>,
    ) -> TickResult<TPosition>;
    fn tick(&mut self) -> TickResult<TPosition>;

    fn run(&mut self, max_iterations: usize) -> TickResult<TPosition> {
        for _ in 0..max_iterations {
            self.tick()?;
        }

        Err(WaveFunctionCollapseInterruption::MaxIterationsReached)
    }
}
