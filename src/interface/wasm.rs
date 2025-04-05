use std::collections::{BTreeSet, HashSet};

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::{
    grid::dynamic_2d::DynamicSizeGrid2D,
    interface::{TileInterface, WaveFunctionCollapse},
    rules::RuleSet2D,
    tile::{Tile, TileState},
    utils::space::{Direction2D, Location2D},
};

#[wasm_bindgen]
pub struct RulePair(TileState, Direction2D, TileState);
#[wasm_bindgen]
impl RulePair {
    #[wasm_bindgen(constructor)]
    pub fn new(a: TileState, dir: Direction2D, b: TileState) -> Self {
        Self(a, dir, b)
    }
}

#[wasm_bindgen]
pub struct Rules(RuleSet2D);

#[wasm_bindgen]
impl Rules {
    #[wasm_bindgen(constructor)]
    pub fn new(possible: Vec<TileState>, allowed: Vec<RulePair>) -> Self {
        let inner = RuleSet2D::new(
            BTreeSet::from_iter(possible),
            HashSet::from_iter(allowed.into_iter().map(|p| (p.0, p.1, p.2))),
        );
        Self(inner)
    }

    pub fn checkers() -> Self {
        let inner = crate::rules::samples::checkers::rules();
        Self(inner)
    }

    pub fn terrain() -> Self {
        let inner = crate::rules::samples::terrain::rules();
        Self(inner)
    }

    pub fn terrain_simple() -> Self {
        let inner = crate::rules::samples::terrain_simple::rules();
        Self(inner)
    }

    pub fn stripes() -> Self {
        let inner = crate::rules::samples::stripes::rules();
        Self(inner)
    }
}

#[wasm_bindgen]
pub struct Grid(DynamicSizeGrid2D);

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(rules: Rules, width: usize, height: usize) -> Self {
        let inner = DynamicSizeGrid2D::new(width, height, rules.0);
        Self(inner)
    }

    pub fn get_dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.0.width,
            height: self.0.height,
        }
    }

    pub fn dump(&self) -> Vec<Tile> {
        self.0.dump()
    }

    pub fn is_finished(&self) -> bool {
        let uncollapsed_tile_exists = self
            .dump()
            .into_iter()
            .any(|t| t.possible_states_ref().count() != 1);
        !uncollapsed_tile_exists
    }

    pub fn collapse(&mut self, x: usize, y: usize, value: Option<TileState>) -> Option<bool> {
        let result = self.0.collapse(Location2D { x, y }, value);
        let done = match result {
            Err(crate::interface::WaveFunctionCollapseInterruption::Finished) => true,
            Err(_) => return None,
            Ok(_) => false,
        };
        Some(done)
    }

    pub fn tick(&mut self) -> Option<bool> {
        let result = self.0.tick();
        let done = match result {
            Err(crate::interface::WaveFunctionCollapseInterruption::Finished) => true,
            Err(_) => return None,
            Ok(_) => false,
        };
        Some(done)
    }

    pub fn run(&mut self, max_iter: usize) -> Option<bool> {
        let result = self.0.run(max_iter);
        let done = match result {
            Err(crate::interface::WaveFunctionCollapseInterruption::Finished) => true,
            Err(_) => return None,
            Ok(_) => false,
        };
        Some(done)
    }
}
