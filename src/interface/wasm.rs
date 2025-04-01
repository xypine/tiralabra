use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use crate::{
    grid::dynamic_2d::DynamicSizeGrid2D,
    interface::{GridInterface, TickResult, WaveFunctionCollapse},
    rules::RuleSet2D,
    tile::Tile,
    utils::space::Location2D,
};

#[wasm_bindgen]
pub struct Grid(DynamicSizeGrid2D);

#[wasm_bindgen]
pub struct Rules(RuleSet2D);

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(rules: Rules, width: usize, height: usize) -> Self {
        let inner = DynamicSizeGrid2D::new(width, height, rules.0);
        Self(inner)
    }

    pub fn image(&self) -> JsValue {
        let map = self.0.image();
        serde_wasm_bindgen::to_value(&map).unwrap()
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
