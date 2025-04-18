//! What tiles are allowed to exists and where

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
};

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::{
    tile::{TileState, interface::TileInterface},
    utils::space::{Direction, Direction2D, NEIGHBOUR_COUNT_2D},
};

/// Describes the tiles that can exist in the output and which ones can be next one another
#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub struct RuleSet<const NEIGHBOURS: usize, TDirection: Direction<NEIGHBOURS>> {
    pub possible: BTreeSet<TileState>,
    /// If (A, RIGHT, B) exists:
    /// - B is allowed on the right side of A
    /// - A is allowed on the left side of B
    pub allowed: HashSet<(TileState, TDirection, TileState)>,
    pub state_representations: HashMap<TileState, String>,
}

pub type RuleSet2D = RuleSet<NEIGHBOUR_COUNT_2D, Direction2D>;

impl<const NEIGHBOURS: usize, TDirection: Direction<NEIGHBOURS> + Hash + Eq + Copy>
    RuleSet<NEIGHBOURS, TDirection>
{
    pub fn new(
        possible: BTreeSet<TileState>,
        allowed: HashSet<(TileState, TDirection, TileState)>,
        state_representations: HashMap<TileState, String>,
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
            state_representations,
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

    /// Returns a value accepted by the css "background" property
    pub fn visualize_tile(&self, state: TileState) -> Option<&String> {
        self.state_representations.get(&state)
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
            let repr = HashMap::from([
                (STATE_BLACK, String::from("#000000")),
                (STATE_WHITE, String::from("#FFFFFF")),
            ]);
            RuleSet::new(possible, allowed, repr)
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
            RuleSet::new(possible, allowed, HashMap::new())
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
            let repr = HashMap::from([
                (STATE_SEA, String::from("#0000ff")),
                (STATE_SHORE, String::from("#fff8dc")),
                (STATE_LAND, String::from("#008000")),
            ]);
            let allowed = HashSet::from([
                // identity rules, allow x next to x
                (STATE_SEA, Direction2D::UP, STATE_SEA),
                (STATE_SEA, Direction2D::RIGHT, STATE_SEA),
                (STATE_SHORE, Direction2D::UP, STATE_SHORE),
                (STATE_SHORE, Direction2D::RIGHT, STATE_SHORE),
                (STATE_LAND, Direction2D::UP, STATE_LAND),
                (STATE_LAND, Direction2D::RIGHT, STATE_LAND),
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
            RuleSet::new(possible, allowed, repr)
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
            let repr = HashMap::from([
                (STATE_DEEP_SEA2, String::from("#000071")),
                (STATE_DEEP_SEA, String::from("#00008b")),
                (STATE_SEA, String::from("#0000ff")),
                (STATE_SHORE, String::from("#fff8dc")),
                (STATE_LAND, String::from("#008000")),
                (STATE_FOREST, String::from("#006400")),
                (STATE_FOREST2, String::from("#005b00")),
            ]);
            let allowed = HashSet::from([
                // identity rules, allow x next to x
                (STATE_DEEP_SEA2, Direction2D::UP, STATE_DEEP_SEA2),
                (STATE_DEEP_SEA2, Direction2D::RIGHT, STATE_DEEP_SEA2),
                (STATE_DEEP_SEA, Direction2D::UP, STATE_DEEP_SEA),
                (STATE_DEEP_SEA, Direction2D::RIGHT, STATE_DEEP_SEA),
                (STATE_SEA, Direction2D::UP, STATE_SEA),
                (STATE_SEA, Direction2D::RIGHT, STATE_SEA),
                (STATE_SHORE, Direction2D::UP, STATE_SHORE),
                (STATE_SHORE, Direction2D::RIGHT, STATE_SHORE),
                (STATE_LAND, Direction2D::UP, STATE_LAND),
                (STATE_LAND, Direction2D::RIGHT, STATE_LAND),
                (STATE_FOREST, Direction2D::UP, STATE_FOREST),
                (STATE_FOREST, Direction2D::RIGHT, STATE_FOREST),
                (STATE_FOREST2, Direction2D::UP, STATE_FOREST2),
                (STATE_FOREST2, Direction2D::RIGHT, STATE_FOREST2),
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
            RuleSet::new(possible, allowed, repr)
        }
    }

    pub mod flowers_singlepixel {
        use super::*;
        pub const STATE_GROUND: u64 = 0;
        pub const STATE_SOIL: u64 = 1;
        pub const STATE_SKY: u64 = 2;
        pub const STATE_STEM: u64 = 3;
        pub const STATE_BRANCH: u64 = 4;
        pub const STATE_CURVE_L: u64 = 8;
        pub const STATE_CURVE_R: u64 = 9;
        pub const STATE_BRANCH_L: u64 = 5;
        pub const STATE_BRANCH_R: u64 = 6;
        pub const STATE_FLOWER: u64 = 7;
        pub fn rules() -> RuleSet2D {
            let possible = BTreeSet::from([
                STATE_GROUND,
                STATE_SOIL,
                STATE_SKY,
                STATE_STEM,
                STATE_BRANCH,
                STATE_BRANCH_L,
                STATE_BRANCH_R,
                STATE_FLOWER,
                STATE_CURVE_L,
                STATE_CURVE_R,
            ]);
            let repr = HashMap::from([
                (STATE_GROUND, String::from("#000000")),
                (STATE_SOIL, String::from("#250500")),
                (STATE_SKY, String::from("#fff8dc")),
                (STATE_STEM, String::from("#006400")),
                (STATE_BRANCH, String::from("#008000")),
                (STATE_BRANCH_L, String::from("#008000")),
                (STATE_BRANCH_R, String::from("#008000")),
                (STATE_FLOWER, String::from("#ffbb55")),
                (STATE_CURVE_L, String::from("#006400")),
                (STATE_CURVE_R, String::from("#006400")),
            ]);
            let allowed = HashSet::from([
                // Allow ground next to ground
                (STATE_GROUND, Direction2D::LEFT, STATE_GROUND),
                (STATE_GROUND, Direction2D::RIGHT, STATE_GROUND),
                // Allow soil on top of ground
                (STATE_SOIL, Direction2D::DOWN, STATE_GROUND),
                // Allow soil next to soil
                (STATE_SOIL, Direction2D::LEFT, STATE_SOIL),
                (STATE_SOIL, Direction2D::RIGHT, STATE_SOIL),
                // Allow stems in soil
                (STATE_STEM, Direction2D::DOWN, STATE_GROUND),
                (STATE_SOIL, Direction2D::LEFT, STATE_STEM),
                (STATE_SOIL, Direction2D::RIGHT, STATE_STEM),
                // Allow stem on top of stem
                (STATE_STEM, Direction2D::DOWN, STATE_STEM),
                (STATE_BRANCH, Direction2D::DOWN, STATE_STEM),
                (STATE_CURVE_L, Direction2D::DOWN, STATE_STEM),
                (STATE_CURVE_R, Direction2D::DOWN, STATE_STEM),
                // Allow branch on sides of stem
                (STATE_BRANCH_L, Direction2D::LEFT, STATE_BRANCH),
                (STATE_BRANCH_L, Direction2D::LEFT, STATE_CURVE_L),
                (STATE_BRANCH_R, Direction2D::RIGHT, STATE_BRANCH),
                (STATE_BRANCH_R, Direction2D::RIGHT, STATE_CURVE_R),
                (STATE_BRANCH_L, Direction2D::DOWN, STATE_SKY),
                (STATE_BRANCH_R, Direction2D::DOWN, STATE_SKY),
                // Allow stem on top of branch
                (STATE_STEM, Direction2D::DOWN, STATE_BRANCH_L),
                (STATE_STEM, Direction2D::DOWN, STATE_BRANCH_R),
                (STATE_BRANCH, Direction2D::DOWN, STATE_BRANCH_L),
                (STATE_BRANCH, Direction2D::DOWN, STATE_BRANCH_R),
                (STATE_CURVE_L, Direction2D::DOWN, STATE_BRANCH_L),
                (STATE_CURVE_L, Direction2D::DOWN, STATE_BRANCH_R),
                (STATE_CURVE_R, Direction2D::DOWN, STATE_BRANCH_L),
                (STATE_CURVE_R, Direction2D::DOWN, STATE_BRANCH_R),
                // Allow sky on top of soil, stem, branch
                (STATE_SKY, Direction2D::DOWN, STATE_SOIL),
                (STATE_SKY, Direction2D::DOWN, STATE_FLOWER),
                (STATE_FLOWER, Direction2D::DOWN, STATE_STEM),
                (STATE_FLOWER, Direction2D::DOWN, STATE_BRANCH_L),
                (STATE_FLOWER, Direction2D::DOWN, STATE_BRANCH_R),
                (STATE_SKY, Direction2D::DOWN, STATE_BRANCH),
                (STATE_SKY, Direction2D::DOWN, STATE_CURVE_L),
                (STATE_SKY, Direction2D::DOWN, STATE_CURVE_R),
                // Allow sky next to sky
                (STATE_SKY, Direction2D::DOWN, STATE_SKY),
                (STATE_SKY, Direction2D::LEFT, STATE_SKY),
                // Allow sky next to stem
                (STATE_SKY, Direction2D::RIGHT, STATE_STEM),
                (STATE_SKY, Direction2D::LEFT, STATE_STEM),
                (STATE_SKY, Direction2D::RIGHT, STATE_BRANCH_R),
                (STATE_SKY, Direction2D::LEFT, STATE_CURVE_R),
                (STATE_SKY, Direction2D::LEFT, STATE_BRANCH_L),
                (STATE_SKY, Direction2D::RIGHT, STATE_CURVE_L),
                (STATE_SKY, Direction2D::RIGHT, STATE_FLOWER),
                (STATE_SKY, Direction2D::LEFT, STATE_FLOWER),
            ]);
            RuleSet::new(possible, allowed, repr)
        }
    }
}
