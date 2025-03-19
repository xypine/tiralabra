use std::collections::HashSet;

use crate::{interface::TileInterface, space::Location2D};

// We can find a better representation later, for now we'll just use the output of the rust hasher
// trait
pub type TileState = u64;

#[derive(Debug, Clone)]
pub struct Tile {
    possible_states: HashSet<TileState>,
}

impl Tile {
    pub fn new(possible: HashSet<TileState>) -> Self {
        Self {
            possible_states: possible,
        }
    }

    pub fn force_set(&mut self, state: TileState) {
        self.possible_states = HashSet::from([state]);
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
}
