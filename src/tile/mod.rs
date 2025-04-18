pub mod simple;

use rand::Rng;

use crate::utils::entropy::Entropy;

pub trait TileInterface<State, TCoords> {
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
