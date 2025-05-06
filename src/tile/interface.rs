use std::collections::HashMap;
use std::hash::Hash;

use rand::Rng;

use crate::utils::entropy::Entropy;

pub enum TileCollapseInstruction<'a, State, R: Rng> {
    Predetermined(State),
    /// rng, weights
    Random(&'a mut R, &'a HashMap<State, usize>),
}

pub trait TileInterface<State: Hash + Eq + Copy> {
    fn new<I: IntoIterator<Item = State>>(possible: I) -> Self;

    /// Returns an iterator over the possible states of the tile.
    /// No data is copied so the usage of this function should be quite efficient
    fn possible_states_ref<'a>(&'a self) -> impl Iterator<Item = &'a State>
    where
        State: 'a;

    /// Returns an iterator over copies of possible states of the tile.
    fn possible_states(&self) -> impl Iterator<Item = State>;

    /// Optimized version of `possible_states_ref.count() == 1`
    fn has_collapsed(&self) -> bool;

    /// "Entropy" is used to pick the best tile to collapse
    /// Basically tiles with few remaining possible states should have low entropy
    // automatically implemented for all types that implement TileInterface
    fn calculate_entropy<R: Rng>(
        &mut self,
        weights: &HashMap<State, usize>,
        rng: &mut R,
    ) -> Option<Entropy>;

    /// Forces the tile into a single state.
    /// If no value is provided, one is chosen from the currently available states.
    /// If no available states exist, `None` is returned
    fn collapse<R: Rng>(&mut self, value: TileCollapseInstruction<State, R>) -> Option<State>;
    fn set_possible_states<I: IntoIterator<Item = State>>(&mut self, states: I);
}
