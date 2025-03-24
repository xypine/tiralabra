use std::collections::HashMap;

use crate::entropy::Entropy;

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
    fn calculate_entropy(&self) -> Entropy {
        // TODO: Replace with the actual entropy calculation
        let possible = self.possible_states().count();
        let entropy = if possible < 2 {
            f64::INFINITY
        } else {
            possible as f64
        };
        Entropy(entropy)
    }

    fn collapse(&mut self) -> Option<State>;
}

pub trait Location<const DIMENSIONS: usize> {}
pub trait Direction<const COUNT: usize> {}

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
    fn get_neighbours(&self, location: TPosition)
    -> [(TDirection, Option<T>); NEIGHBOURS_PER_TILE];

    /// Fetches the tile at the given location and gives you mutable access to it
    fn with_tile<R, F: Fn(&mut T) -> R>(&mut self, location: TPosition, f: F) -> Option<R>;
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum WaveFunctionCollapseInterruption<TPosition> {
    Finished,
    Contradiction(TPosition),
    MaxIterationsReached,
}

pub type TickResult<TPosition> = Result<(), WaveFunctionCollapseInterruption<TPosition>>;
pub trait WaveFunctionCollapse<TPosition> {
    fn find_lowest_entropy(&mut self) -> Option<TPosition>;
    fn collapse(
        &mut self,
        position: TPosition,
    ) -> Result<(), WaveFunctionCollapseInterruption<TPosition>>;
    fn tick(&mut self) -> TickResult<TPosition>;

    fn run(&mut self, max_iterations: usize) -> TickResult<TPosition> {
        for _ in 0..max_iterations {
            self.tick()?;
        }

        Err(WaveFunctionCollapseInterruption::MaxIterationsReached)
    }
}
