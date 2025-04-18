pub mod grid;
pub mod rules;
pub mod tile;
pub mod utils;

pub mod tile_extraction;
pub mod wave_function_collapse;

#[cfg(not(tarpaulin_include))] // the wasm bindings don't need to be unit tested
pub mod wasm;
