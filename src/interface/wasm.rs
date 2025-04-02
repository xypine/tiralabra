use std::collections::{BTreeSet, HashSet};

use wasm_bindgen::prelude::*;

use crate::{
    grid::dynamic_2d::DynamicSizeGrid2D,
    interface::WaveFunctionCollapse,
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
}

#[wasm_bindgen]
pub struct Grid(DynamicSizeGrid2D);

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(rules: Rules, width: usize, height: usize) -> Self {
        let inner = DynamicSizeGrid2D::new(width, height, rules.0);
        Self(inner)
    }

    pub fn dump(&self) -> Vec<Tile> {
        self.0.dump()
    }

    pub fn collapse(&mut self, x: usize, y: usize) -> Option<bool> {
        let result = self.0.collapse(Location2D { x, y });
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
}
