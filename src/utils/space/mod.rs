//! Dimension-agnostic interfaces for space-related utils

use std::fmt::Debug;
use std::hash::Hash;

pub mod s2d;
// 1d or 3d versions are not a part of the core algorithm
// as such, they won't be unit tested
#[cfg(not(tarpaulin_include))]
pub mod s1d;
#[cfg(not(tarpaulin_include))]
pub mod s3d;

/// Dimension-agnostic Location
pub trait Location: Debug + Hash + Eq + Copy + Ord {
    /// returns the distance from zero
    fn length(&self) -> usize;
}

/// Dimension-agnostic direction that can be mirrored
///
/// used for finding neighbours of tiles
pub trait Direction<const COUNT: usize>: Hash + Eq + Ord {
    fn mirror(self) -> Self;
}
