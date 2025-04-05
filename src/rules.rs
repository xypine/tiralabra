//! What tiles are allowed to exists and where

use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
};

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::{
    interface::{Direction, TileInterface},
    tile::TileState,
    utils::space::{Direction2D, NEIGHBOUR_COUNT_2D},
};

/// Describes the tiles that can exist in the output and which ones can be next one another
#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub struct RuleSet<const NEIGHBOURS: usize, TDirection: Direction<NEIGHBOURS>> {
    pub possible: BTreeSet<TileState>,
    /// If (A, RIGHT, B) exists:
    /// - B is allowed on the right side of A
    /// - A is allowed on the left side of B
    pub allowed: HashSet<(TileState, TDirection, TileState)>,
}

pub type RuleSet2D = RuleSet<NEIGHBOUR_COUNT_2D, Direction2D>;

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

    /// Removes possible tile states for `target`,
    /// given that it has a neighbour `source` in `direction`
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

pub mod samples {
    use super::*;

    /// Allows a checker or "chess board" pattern, with alternating white and black tiles
    pub mod checkers {
        use super::*;
        pub const STATE_BLACK: u64 = 0;
        pub const STATE_WHITE: u64 = 1;
        pub fn rules() -> RuleSet2D {
            let possible = BTreeSet::from([STATE_BLACK, STATE_WHITE]);
            let allowed = HashSet::from([
                (STATE_BLACK, Direction2D::UP, STATE_WHITE),
                (STATE_BLACK, Direction2D::RIGHT, STATE_WHITE),
                (STATE_BLACK, Direction2D::DOWN, STATE_WHITE),
                (STATE_BLACK, Direction2D::LEFT, STATE_WHITE),
            ]);
            RuleSet::new(possible, allowed)
        }
    }

    /// Three colors alternating diagonally
    pub mod stripes {
        use super::*;
        pub const STATE_ONE: u64 = 2;
        pub const STATE_MIDDLE: u64 = 3;
        pub const STATE_TWO: u64 = 4;
        pub fn rules() -> RuleSet2D {
            let possible = BTreeSet::from([STATE_ONE, STATE_MIDDLE, STATE_TWO]);
            let allowed = HashSet::from([
                // adjacency rules, allow ONE on top and left of MIDDLE, TWO on right and bottom
                (STATE_ONE, Direction2D::DOWN, STATE_MIDDLE),
                (STATE_ONE, Direction2D::RIGHT, STATE_MIDDLE),
                (STATE_MIDDLE, Direction2D::DOWN, STATE_TWO),
                (STATE_MIDDLE, Direction2D::RIGHT, STATE_TWO),
                (STATE_TWO, Direction2D::DOWN, STATE_ONE),
                (STATE_TWO, Direction2D::RIGHT, STATE_ONE),
            ]);
            RuleSet::new(possible, allowed)
        }
    }

    /// Sea -> Shore -> Land
    /// directions do not matter, but the order must be as above
    /// "Sea" should never end up next to "Land"
    pub mod terrain_simple {
        use super::*;
        const STATE_SEA: u64 = 2;
        const STATE_SHORE: u64 = 3;
        const STATE_LAND: u64 = 4;
        pub fn rules() -> RuleSet2D {
            let possible = BTreeSet::from([STATE_SEA, STATE_SHORE, STATE_LAND]);
            let allowed = HashSet::from([
                // identity rules, allow x next to x
                (STATE_SEA, Direction2D::UP, STATE_SEA),
                (STATE_SEA, Direction2D::RIGHT, STATE_SEA),
                (STATE_SEA, Direction2D::DOWN, STATE_SEA),
                (STATE_SEA, Direction2D::LEFT, STATE_SEA),
                (STATE_SHORE, Direction2D::UP, STATE_SHORE),
                (STATE_SHORE, Direction2D::RIGHT, STATE_SHORE),
                (STATE_SHORE, Direction2D::DOWN, STATE_SHORE),
                (STATE_SHORE, Direction2D::LEFT, STATE_SHORE),
                (STATE_LAND, Direction2D::UP, STATE_LAND),
                (STATE_LAND, Direction2D::RIGHT, STATE_LAND),
                (STATE_LAND, Direction2D::DOWN, STATE_LAND),
                (STATE_LAND, Direction2D::LEFT, STATE_LAND),
                // adjacency rules, allow SEA -> SHORE -> LAND
                (STATE_SEA, Direction2D::UP, STATE_SHORE),
                (STATE_SEA, Direction2D::RIGHT, STATE_SHORE),
                (STATE_SEA, Direction2D::DOWN, STATE_SHORE),
                (STATE_SEA, Direction2D::LEFT, STATE_SHORE),
                (STATE_SHORE, Direction2D::UP, STATE_LAND),
                (STATE_SHORE, Direction2D::RIGHT, STATE_LAND),
                (STATE_SHORE, Direction2D::DOWN, STATE_LAND),
                (STATE_SHORE, Direction2D::LEFT, STATE_LAND),
            ]);
            RuleSet::new(possible, allowed)
        }
    }

    /// Deepest sea -> Deep sea -> Sea -> Shore -> Land -> Forest -> Deep forest
    /// directions do not matter, but the order must be as above
    pub mod terrain {
        use super::*;
        const STATE_DEEP_SEA2: u64 = 0;
        const STATE_DEEP_SEA: u64 = 1;
        const STATE_SEA: u64 = 2;
        const STATE_SHORE: u64 = 3;
        const STATE_LAND: u64 = 4;
        const STATE_FOREST: u64 = 5;
        const STATE_FOREST2: u64 = 6;
        pub fn rules() -> RuleSet2D {
            let possible = BTreeSet::from([
                STATE_DEEP_SEA2,
                STATE_DEEP_SEA,
                STATE_SEA,
                STATE_SHORE,
                STATE_LAND,
                STATE_FOREST,
                STATE_FOREST2,
            ]);
            let allowed = HashSet::from([
                // identity rules, allow x next to x
                (STATE_DEEP_SEA2, Direction2D::UP, STATE_DEEP_SEA2),
                (STATE_DEEP_SEA2, Direction2D::RIGHT, STATE_DEEP_SEA2),
                (STATE_DEEP_SEA2, Direction2D::DOWN, STATE_DEEP_SEA2),
                (STATE_DEEP_SEA2, Direction2D::LEFT, STATE_DEEP_SEA2),
                (STATE_DEEP_SEA, Direction2D::UP, STATE_DEEP_SEA),
                (STATE_DEEP_SEA, Direction2D::RIGHT, STATE_DEEP_SEA),
                (STATE_DEEP_SEA, Direction2D::DOWN, STATE_DEEP_SEA),
                (STATE_DEEP_SEA, Direction2D::LEFT, STATE_DEEP_SEA),
                (STATE_SEA, Direction2D::UP, STATE_SEA),
                (STATE_SEA, Direction2D::RIGHT, STATE_SEA),
                (STATE_SEA, Direction2D::DOWN, STATE_SEA),
                (STATE_SEA, Direction2D::LEFT, STATE_SEA),
                (STATE_SHORE, Direction2D::UP, STATE_SHORE),
                (STATE_SHORE, Direction2D::RIGHT, STATE_SHORE),
                (STATE_SHORE, Direction2D::DOWN, STATE_SHORE),
                (STATE_SHORE, Direction2D::LEFT, STATE_SHORE),
                (STATE_LAND, Direction2D::UP, STATE_LAND),
                (STATE_LAND, Direction2D::RIGHT, STATE_LAND),
                (STATE_LAND, Direction2D::DOWN, STATE_LAND),
                (STATE_LAND, Direction2D::LEFT, STATE_LAND),
                (STATE_FOREST, Direction2D::UP, STATE_FOREST),
                (STATE_FOREST, Direction2D::RIGHT, STATE_FOREST),
                (STATE_FOREST, Direction2D::DOWN, STATE_FOREST),
                (STATE_FOREST, Direction2D::LEFT, STATE_FOREST),
                (STATE_FOREST2, Direction2D::UP, STATE_FOREST2),
                (STATE_FOREST2, Direction2D::RIGHT, STATE_FOREST2),
                (STATE_FOREST2, Direction2D::DOWN, STATE_FOREST2),
                (STATE_FOREST2, Direction2D::LEFT, STATE_FOREST2),
                // adjacency rules, allow DEEP_SEA -> SEA -> SHORE -> LAND -> FOREST
                (STATE_DEEP_SEA2, Direction2D::UP, STATE_DEEP_SEA),
                (STATE_DEEP_SEA2, Direction2D::RIGHT, STATE_DEEP_SEA),
                (STATE_DEEP_SEA2, Direction2D::DOWN, STATE_DEEP_SEA),
                (STATE_DEEP_SEA2, Direction2D::LEFT, STATE_DEEP_SEA),
                (STATE_DEEP_SEA, Direction2D::UP, STATE_SEA),
                (STATE_DEEP_SEA, Direction2D::RIGHT, STATE_SEA),
                (STATE_DEEP_SEA, Direction2D::DOWN, STATE_SEA),
                (STATE_DEEP_SEA, Direction2D::LEFT, STATE_SEA),
                (STATE_SEA, Direction2D::UP, STATE_SHORE),
                (STATE_SEA, Direction2D::RIGHT, STATE_SHORE),
                (STATE_SEA, Direction2D::DOWN, STATE_SHORE),
                (STATE_SEA, Direction2D::LEFT, STATE_SHORE),
                (STATE_SHORE, Direction2D::UP, STATE_LAND),
                (STATE_SHORE, Direction2D::RIGHT, STATE_LAND),
                (STATE_SHORE, Direction2D::DOWN, STATE_LAND),
                (STATE_SHORE, Direction2D::LEFT, STATE_LAND),
                (STATE_LAND, Direction2D::UP, STATE_FOREST),
                (STATE_LAND, Direction2D::RIGHT, STATE_FOREST),
                (STATE_LAND, Direction2D::DOWN, STATE_FOREST),
                (STATE_LAND, Direction2D::LEFT, STATE_FOREST),
                (STATE_FOREST, Direction2D::UP, STATE_FOREST2),
                (STATE_FOREST, Direction2D::RIGHT, STATE_FOREST2),
                (STATE_FOREST, Direction2D::DOWN, STATE_FOREST2),
                (STATE_FOREST, Direction2D::LEFT, STATE_FOREST2),
            ]);
            RuleSet::new(possible, allowed)
        }
    }
}
