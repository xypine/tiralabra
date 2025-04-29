//! Interface and structures that can be used in the browser through Web Assembly
//! TypeScript types are automatically generated using Tsify

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::{
    backtracking::reset::BacktrackerByReset,
    grid::dynamic_2d::DynamicSizeGrid2D,
    rules::RuleSet2D,
    tile::{Tile, TileState, interface::TileInterface},
    utils::{
        render::CanvasRenderable,
        space::s2d::{Direction2D, Location2D},
    },
    wave_function_collapse::interface::{WaveFunctionCollapse, WaveFunctionCollapseInterruption},
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

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
pub struct TileVisual(TileState, Option<String>);

#[wasm_bindgen]
pub struct Rules(RuleSet2D);

#[wasm_bindgen]
impl Rules {
    #[wasm_bindgen(constructor)]
    pub fn new(
        possible: Vec<TileState>,
        allowed: Vec<RulePair>,
        possible_weights: Vec<usize>,
        possible_repr: Vec<u32>,
    ) -> Self {
        let mut repr = HashMap::new();
        let mut weights = HashMap::new();
        for (i, state) in possible.iter().enumerate() {
            if let Some(state_repr) = possible_repr.get(i).cloned() {
                repr.insert(*state, state_repr);
            }
            if let Some(state_weight) = possible_weights.get(i).cloned() {
                weights.insert(*state, state_weight);
            }
        }
        let inner = RuleSet2D::new(
            BTreeSet::from_iter(possible),
            HashSet::from_iter(allowed.into_iter().map(|p| (p.0, p.1, p.2))),
            weights,
            repr,
            BTreeMap::new(),
        );
        Self(inner)
    }

    pub fn check(
        &self,
        target_tile_states: Vec<TileState>,
        source_tile_states: Vec<TileState>,
        direction: Direction2D,
    ) -> Vec<TileState> {
        let target = Tile::new(BTreeSet::from_iter(target_tile_states));
        let source = Tile::new(BTreeSet::from_iter(source_tile_states));
        self.0
            .check(&target, &source, direction)
            .into_iter()
            .collect()
    }

    pub fn get_visual_tileset(&self) -> Vec<TileVisual> {
        self.0
            .possible
            .iter()
            .map(|state| (state, self.0.visualize_tile(*state)))
            .map(|(state, color)| {
                let v = color.map(|color| {
                    let a = ((color >> 24) & 0xFF) as u8;
                    let r = ((color >> 16) & 0xFF) as u8;
                    let g = ((color >> 8) & 0xFF) as u8;
                    let b = (color & 0xFF) as u8;
                    if a == 255 {
                        format!("rgb({r},{g},{b})")
                    } else {
                        let alpha = (a as f32) / 255.0;
                        format!("rgba({r},{g},{b},{alpha:.3})")
                    }
                });
                TileVisual(*state, v)
            })
            .collect()
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

    pub fn flowers_singlepixel() -> Self {
        let inner = crate::rules::samples::flowers_singlepixel::rules();
        Self(inner)
    }

    pub fn flowers() -> Self {
        let inner: RuleSet2D = serde_json::from_str(include_str!("../rules.json"))
            .expect("failed to parse prebuilt rules.json");
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
    pub fn new(rng_seed: u64, rules: Rules, width: usize, height: usize) -> Self {
        console_error_panic_hook::set_once();
        let inner = DynamicSizeGrid2D::new(width, height, rules.0, rng_seed);
        Self(inner)
    }

    pub fn get_dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.0.width,
            height: self.0.height,
        }
    }

    pub fn render(&self, w: usize, h: usize) -> String {
        self.0.render(w, h)
    }

    pub fn dump(&self) -> Vec<Tile> {
        self.0.dump()
    }

    pub fn get_history_len(&self) -> usize {
        self.0.update_log.len()
    }

    pub fn dump_at_time(&self, t: usize) -> Vec<Tile> {
        self.0.dump_at_time(t)
    }

    pub fn is_finished(&self) -> bool {
        let uncollapsed_tile_exists = self
            .0
            .tiles_ref()
            .iter()
            .any(|t| t.possible_states_ref().count() != 1);
        !uncollapsed_tile_exists
    }

    pub fn collapse(&mut self, x: usize, y: usize, value: Option<TileState>) -> Option<bool> {
        let result = self.0.collapse(Location2D { x, y }, value);
        let done = match result {
            Err(WaveFunctionCollapseInterruption::Finished) => true,
            Err(_) => return None,
            Ok(_) => false,
        };
        Some(done)
    }

    pub fn tick(&mut self) -> Option<bool> {
        let result = self.0.run(1, Some(BacktrackerByReset {}));
        let done = match result {
            Err(WaveFunctionCollapseInterruption::Finished) => true,
            Err(WaveFunctionCollapseInterruption::MaxIterationsReached) => false,
            Err(_) => return None,
            Ok(_) => false,
        };
        Some(done)
    }

    pub fn run(&mut self, max_iter: usize) -> Option<bool> {
        let result = self.0.run(max_iter, Some(BacktrackerByReset {}));
        let done = match result {
            Err(WaveFunctionCollapseInterruption::Finished) => true,
            Err(_) => return None,
            Ok(_) => false,
        };
        Some(done)
    }
}
