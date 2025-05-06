//! Simple tile implementation
//!
//! Possible states are stored in a BTreeSet

pub mod interface;

use std::collections::BTreeSet;

use interface::{TileCollapseInstruction, TileInterface};
use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
};
use serde::{Deserialize, Serialize};
use tsify_next::{Tsify, declare};

use crate::utils::entropy::Entropy;

/// Represents a possible state that any tile in the grid can be collapsed into
// We can find a better representation later, for now we'll just use the output of the rust hasher
// trait
#[declare]
pub type TileState = u64;

#[derive(Debug, Clone, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
pub struct Tile {
    possible_states: BTreeSet<TileState>,
    // can be calculated from possible_states, but we can spare some memory for better performance
    collapsed: bool,
    // same here
    entropy: Option<Entropy>,
}

impl Tile {
    #[inline]
    fn invalidate_cache(&mut self) {
        self.collapsed = self.possible_states.len() == 1;
        self.entropy = None;
    }
}

impl TileInterface<TileState> for Tile {
    fn new<I: IntoIterator<Item = TileState>>(possible: I) -> Self {
        let mut new = Self {
            possible_states: BTreeSet::from_iter(possible),
            collapsed: false,
            entropy: None,
        };

        new.invalidate_cache();

        new
    }

    #[inline]
    fn possible_states_ref<'a>(&'a self) -> impl Iterator<Item = &'a TileState>
    where
        TileState: 'a,
    {
        self.possible_states.iter()
    }

    #[inline]
    fn possible_states(&self) -> impl Iterator<Item = TileState> {
        self.possible_states.iter().cloned()
    }

    #[inline]
    fn has_collapsed(&self) -> bool {
        self.collapsed
    }

    #[inline]
    fn collapse<R: Rng>(
        &mut self,
        value: TileCollapseInstruction<TileState, R>,
    ) -> Option<TileState> {
        let chosen_state = match value {
            TileCollapseInstruction::Predetermined(value) => value,
            TileCollapseInstruction::Random(rng, weights) => {
                let w: Vec<_> = self
                    .possible_states_ref()
                    .map(|s| weights.get(s).map(|&w| w as f64).unwrap_or(1.0))
                    .collect();
                let dist = WeightedIndex::new(w).unwrap();
                let chosen_index = dist.sample(rng);
                self.possible_states().nth(chosen_index)?
            }
        };

        self.set_possible_states([chosen_state]);
        Some(chosen_state)
    }

    #[inline]
    fn set_possible_states<I: IntoIterator<Item = TileState>>(&mut self, states: I) {
        self.possible_states = BTreeSet::from_iter(states);
        self.invalidate_cache();
    }

    #[inline]
    fn calculate_entropy<R: Rng>(
        &mut self,
        weights: &std::collections::HashMap<TileState, usize>,
        rng: &mut R,
    ) -> Option<Entropy> {
        if self.has_collapsed() {
            return None;
        }
        if let Some(cached) = self.entropy {
            return Some(cached);
        }
        let w: Vec<_> = self
            .possible_states_ref()
            .map(|s| weights.get(s).map(|&w| w as f64).unwrap_or(1.0))
            .collect();

        let sum: f64 = w.iter().sum();
        if sum == 0.0 {
            // No valid states
            return Some(Entropy(0.0));
        }

        let term1 = sum.ln();
        let term2 = w
            .iter()
            .filter(|&&wi| wi > 0.0)
            .map(|&wi| wi * wi.ln())
            .sum::<f64>()
            / sum;

        let entropy = term1 - term2;
        let noise = rng.random::<f64>() * f64::EPSILON;
        let to_cache = Entropy(entropy + noise);
        self.entropy = Some(to_cache);
        Some(to_cache)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn entropy_calculation_sanity() {
        let mut tile_0_states = Tile::new(BTreeSet::from([]));
        assert!(!tile_0_states.has_collapsed());
        let mut tile_1_states = Tile::new(BTreeSet::from([1]));
        assert!(tile_1_states.has_collapsed());
        let mut tile_2_states = Tile::new(BTreeSet::from([1, 2]));
        assert!(!tile_2_states.has_collapsed());
        let mut tile_3_states = Tile::new(BTreeSet::from([1, 2, 3]));
        let mut tile_4_states_massive_weight = Tile::new(BTreeSet::from([1, 2, 3, 4]));
        assert!(!tile_4_states_massive_weight.has_collapsed());

        let mut rng = rand::rng();
        let weights = HashMap::from([(4, 1000)]);

        // tiles with one state cannot be collapsed
        assert_eq!(tile_1_states.calculate_entropy(&weights, &mut rng), None);
        // otherwise tiles with less states should have a lower entropy
        // (at least with the naive entropy implementation)
        assert!(
            tile_0_states
                .calculate_entropy(&weights, &mut rng)
                .expect("tile with 0 states should have zero entropy")
                < tile_2_states
                    .calculate_entropy(&weights, &mut rng)
                    .expect("tile with 2 states should have some entropy"),
            "tile with 0 states should have a lower entropy than one with 2 states"
        );
        assert!(
            tile_2_states
                .calculate_entropy(&weights, &mut rng)
                .expect("tile with 0 states should have zero entropy")
                < tile_3_states
                    .calculate_entropy(&weights, &mut rng)
                    .expect("tile with 2 states should have some entropy"),
            "tile with 2 states should have a lower entropy than one with 3 states"
        );

        assert!(
            tile_4_states_massive_weight
                .calculate_entropy(&weights, &mut rng)
                .expect("tile with 0 states should have zero entropy")
                < tile_2_states
                    .calculate_entropy(&weights, &mut rng)
                    .expect("tile with 2 states should have some entropy"),
            "tile with 4 states (one having massive weight) should have a lower entropy than one with 2 states (but low weight)"
        );
    }
}
