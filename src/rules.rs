use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
};

use crate::{
    interface::{Direction, TileInterface},
    tile::TileState,
};

#[derive(Debug, Clone)]
pub struct RuleSet<const NEIGHBOURS: usize, TDirection: Direction<NEIGHBOURS> + Hash + Eq> {
    pub possible: BTreeSet<TileState>,
    // direction agnostic, also contains mirrored pairs
    pub allowed: HashSet<(TileState, TDirection, TileState)>,
}

impl<const NEIGHBOURS: usize, TDirection: Direction<NEIGHBOURS> + Hash + Eq + Copy>
    RuleSet<NEIGHBOURS, TDirection>
{
    pub fn new(
        possible: BTreeSet<TileState>,
        allowed: HashSet<(TileState, TDirection, TileState)>,
    ) -> Self {
        let mut allowed_with_mirrored = HashSet::new();
        for entry in allowed {
            allowed_with_mirrored.insert(entry);
            let mirrored = (entry.2, entry.1.mirror(), entry.0);
            allowed_with_mirrored.insert(mirrored);
        }
        Self {
            possible,
            allowed: allowed_with_mirrored,
        }
    }

    pub fn check<TCoords, T: TileInterface<TileState, TCoords>>(
        &self,
        target: &T,
        source: &T,
        direction: TDirection,
    ) -> BTreeSet<TileState> {
        let source_states = source.possible_states_ref();
        let mut checked_possible = BTreeSet::new();
        for caller_state in source_states {
            let currently_possible = target.possible_states_ref();
            for state in currently_possible {
                if self.allowed.contains(&(*state, direction, *caller_state)) {
                    checked_possible.insert(*state);
                }
            }
        }

        checked_possible
    }
}
