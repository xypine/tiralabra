use std::collections::HashSet;

use crate::{interface::TileInterface, space::Location2D};

// We can find a better representation later, for now we'll just use the output of the rust hasher
// trait
pub type TileState = u64;

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    possible_states: HashSet<TileState>,
    // can be calculated from possible_states, but we can spare some memory for better performance
    collapsed: bool,
}

impl Tile {
    #[inline]
    fn invalidate_cache(&mut self) {
        self.collapsed = self.possible_states.len() == 1;
    }

    pub fn new(possible: HashSet<TileState>) -> Self {
        let mut new = Self {
            possible_states: possible,
            collapsed: false,
        };

        new.invalidate_cache();

        new
    }

    pub fn set_possible_states(&mut self, states: HashSet<TileState>) {
        self.possible_states = states;
        self.invalidate_cache();
    }
}

impl TileInterface<TileState, Location2D> for Tile {
    fn possible_states(&self) -> impl Iterator<Item = TileState> {
        self.possible_states.iter().cloned()
    }
    fn possible_states_ref<'a>(&'a self) -> impl Iterator<Item = &'a TileState>
    where
        TileState: 'a,
    {
        self.possible_states.iter()
    }

    fn has_collapsed(&self) -> bool {
        self.collapsed
    }

    fn collapse(&mut self) -> Option<TileState> {
        let chosen_state = self.possible_states().next()?;
        self.set_possible_states(HashSet::from([chosen_state]));
        Some(chosen_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entropy_calculation_sanity() {
        let tile_0_states = Tile::new(HashSet::from([]));
        assert!(!tile_0_states.has_collapsed());
        let tile_1_states = Tile::new(HashSet::from([1]));
        assert!(tile_1_states.has_collapsed());
        let tile_2_states = Tile::new(HashSet::from([1, 2]));
        assert!(!tile_2_states.has_collapsed());
        let tile_3_states = Tile::new(HashSet::from([1, 2, 3]));
        assert!(!tile_3_states.has_collapsed());

        // tiles with zero or one state(s) cannot be collapsed
        assert!(tile_0_states.calculate_entropy() == tile_1_states.calculate_entropy());
        // otherwise tiles with less states should have a lower entropy
        // (at least with the naive entropy implementation)
        assert!(tile_2_states.calculate_entropy() < tile_3_states.calculate_entropy());
    }
}
