//! Grid implementations and common test components for them

use std::collections::HashMap;

use crate::{rules::RuleSet, tile::interface::TileInterface, utils::space::Direction};

pub mod constant_2d;
pub mod dynamic_2d;

#[cfg(test)]
pub mod tests;

/// Dimension-agnostic container for tiles
pub trait GridInterface<
    const NEIGHBOURS_PER_TILE: usize,
    TState,
    TPosition,
    TDirection: Direction<{ NEIGHBOURS_PER_TILE }>,
    T: TileInterface<TState, TPosition>,
>: Sized
{
    fn reset(&mut self);

    /// Useful for visuals, might not be most performant
    fn image(&self) -> HashMap<TPosition, T>;

    /// Returns a the requested tile if `location` falls inside the bounds of the grid
    fn get_tile(&self, location: TPosition) -> Option<T>;

    /// Returns an array of locations neighbouring the tile, each can be None if it falls outside
    /// the grid
    fn get_neighbours(
        &self,
        location: TPosition,
    ) -> [(TDirection, Option<TPosition>); NEIGHBOURS_PER_TILE];

    /// Returns an array of the tile's neighbours. Each can be none if the tile has no neighbour in
    /// that direction.
    fn get_neighbour_tiles(
        &self,
        location: TPosition,
    ) -> [(TDirection, Option<T>); NEIGHBOURS_PER_TILE];

    /// Returns the position of the tile in the grid with the lowest "Entropy"
    fn get_lowest_entropy_position(&mut self) -> Option<TPosition>;

    /// Fetches the tile at the given location and gives you mutable access to it
    fn with_tile<R, F: Fn(&mut T) -> R>(&mut self, location: TPosition, f: F) -> Option<R>;

    /// Returns the rules associated with the grid. A grid must know the rules to initialize tiles
    /// correctly.
    fn get_rules(&self) -> RuleSet<NEIGHBOURS_PER_TILE, TDirection>;
}
