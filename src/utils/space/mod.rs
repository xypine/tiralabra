//! Dimension-agnostic interfaces for space-related utils

use std::hash::Hash;

pub mod s1d;
pub mod s2d;
pub mod s3d;

/// Dimension-agnostic Location
pub trait Location<const DIMENSIONS: usize> {}

/// Dimension-agnostic direction that can be mirrored
///
/// used for finding neighbours of tiles
pub trait Direction<const COUNT: usize>: Hash + Eq + Ord {
    fn mirror(self) -> Self;
}
