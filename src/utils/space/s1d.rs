//! One dimensional space

use std::{
    hash::Hash,
    num::TryFromIntError,
    ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use tsify_next::declare;

use super::{Direction, Location};

pub const AXIS_1D: usize = 1;
pub const NEIGHBOUR_COUNT_1D: usize = 2 * AXIS_1D;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Tsify, Serialize, Deserialize)]
pub struct Vector1D<T: Copy> {
    pub x: T,
}

#[declare]
pub type Location1D = Vector1D<usize>;
impl Location1D {
    pub fn try_apply(self, delta: Delta1D) -> Result<Self, TryFromIntError> {
        let self_as_delta = Delta1D::try_from(self)?;
        let result = self_as_delta + delta;
        Self::try_from(result)
    }

    pub fn delta(self, other: Self) -> Result<Delta1D, TryFromIntError> {
        let self_as_delta = Delta1D::try_from(self)?;
        let other_as_delta = Delta1D::try_from(other)?;
        Ok(other_as_delta - self_as_delta)
    }
}

impl Location for Location1D {
    fn length(&self) -> usize {
        self.x
    }
}

impl TryFrom<Location1D> for Delta1D {
    type Error = TryFromIntError;

    fn try_from(value: Location1D) -> Result<Self, Self::Error> {
        let x = isize::try_from(value.x)?;
        Ok(Self { x })
    }
}
impl TryFrom<Delta1D> for Location1D {
    type Error = TryFromIntError;

    fn try_from(value: Delta1D) -> Result<Self, Self::Error> {
        let x = usize::try_from(value.x)?;
        Ok(Self { x })
    }
}

pub type Delta1D = Vector1D<isize>;

impl Add<Delta1D> for Delta1D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x }
    }
}
impl Sub<Delta1D> for Delta1D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x - rhs.x }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Tsify, Serialize, Deserialize,
)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Direction1D {
    RIGHT = 0,
    LEFT = 1,
}

impl TryFrom<usize> for Direction1D {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::RIGHT),
            1 => Ok(Self::LEFT),
            _ => Err(()),
        }
    }
}

impl From<Direction1D> for Delta1D {
    fn from(value: Direction1D) -> Self {
        match value {
            Direction1D::RIGHT => Delta1D { x: 1 },
            Direction1D::LEFT => Delta1D { x: -1 },
        }
    }
}

impl TryFrom<Delta1D> for Direction1D {
    type Error = ();

    fn try_from(value: Delta1D) -> Result<Self, Self::Error> {
        match value {
            Delta1D { x: 1 } => Ok(Direction1D::RIGHT),
            Delta1D { x: -1 } => Ok(Direction1D::LEFT),
            _ => Err(()),
        }
    }
}

impl Direction<NEIGHBOUR_COUNT_1D> for Direction1D {
    fn mirror(self) -> Self {
        match self {
            Direction1D::RIGHT => Direction1D::LEFT,
            Direction1D::LEFT => Direction1D::RIGHT,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Delta1D, Direction1D, Location1D};

    #[test]
    fn delta_sanity() {
        let down = Direction1D::RIGHT;
        let down_delta = Delta1D::from(down);
        let down_position = Location1D::try_from(down_delta).unwrap();
        let zero = Location1D { x: 0 };
        let reconstructed_delta = zero.delta(down_position).unwrap();
        assert_eq!(down_delta, reconstructed_delta);
    }
}
