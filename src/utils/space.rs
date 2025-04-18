use std::{
    hash::Hash,
    num::TryFromIntError,
    ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use tsify_next::declare;

/// Dimension-agnostic Location
pub trait Location<const DIMENSIONS: usize> {}

/// Dimension-agnostic direction that can be mirrored
///
/// used for finding neighbours of tiles
pub trait Direction<const COUNT: usize>: Hash + Eq {
    fn mirror(self) -> Self;
}

pub const AXIS_2D: usize = 2;
pub const NEIGHBOUR_COUNT_2D: usize = 2 * AXIS_2D;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Tsify, Serialize, Deserialize)]
pub struct Vector2D<T: Copy> {
    pub x: T,
    pub y: T,
}

#[declare]
pub type Location2D = Vector2D<usize>;
impl Location2D {
    pub fn try_apply(self, delta: Delta2D) -> Result<Self, TryFromIntError> {
        let self_as_delta = Delta2D::try_from(self)?;
        let result = self_as_delta + delta;
        Self::try_from(result)
    }

    pub fn delta(self, other: Self) -> Result<Delta2D, TryFromIntError> {
        let self_as_delta = Delta2D::try_from(self)?;
        let other_as_delta = Delta2D::try_from(other)?;
        Ok(other_as_delta - self_as_delta)
    }
}

impl Location<AXIS_2D> for Location2D {}

impl TryFrom<Location2D> for Delta2D {
    type Error = TryFromIntError;

    fn try_from(value: Location2D) -> Result<Self, Self::Error> {
        let x = isize::try_from(value.x)?;
        let y = isize::try_from(value.y)?;
        Ok(Self { x, y })
    }
}
impl TryFrom<Delta2D> for Location2D {
    type Error = TryFromIntError;

    fn try_from(value: Delta2D) -> Result<Self, Self::Error> {
        let x = usize::try_from(value.x)?;
        let y = usize::try_from(value.y)?;
        Ok(Self { x, y })
    }
}

pub type Delta2D = Vector2D<isize>;

impl Add<Delta2D> for Delta2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub<Delta2D> for Delta2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Direction2D {
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
}

impl TryFrom<usize> for Direction2D {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UP),
            1 => Ok(Self::RIGHT),
            2 => Ok(Self::DOWN),
            3 => Ok(Self::LEFT),
            _ => Err(()),
        }
    }
}

impl From<Direction2D> for Delta2D {
    fn from(value: Direction2D) -> Self {
        match value {
            Direction2D::UP => Delta2D { x: 0, y: -1 },
            Direction2D::RIGHT => Delta2D { x: 1, y: 0 },
            Direction2D::DOWN => Delta2D { x: 0, y: 1 },
            Direction2D::LEFT => Delta2D { x: -1, y: 0 },
        }
    }
}

impl TryFrom<Delta2D> for Direction2D {
    type Error = ();

    fn try_from(value: Delta2D) -> Result<Self, Self::Error> {
        match value {
            Delta2D { x: 0, y: -1 } => Ok(Direction2D::UP),
            Delta2D { x: 1, y: 0 } => Ok(Direction2D::RIGHT),
            Delta2D { x: 0, y: 1 } => Ok(Direction2D::DOWN),
            Delta2D { x: -1, y: 0 } => Ok(Direction2D::LEFT),
            _ => Err(()),
        }
    }
}

impl Direction<NEIGHBOUR_COUNT_2D> for Direction2D {
    fn mirror(self) -> Self {
        match self {
            Direction2D::UP => Direction2D::DOWN,
            Direction2D::RIGHT => Direction2D::LEFT,
            Direction2D::DOWN => Direction2D::UP,
            Direction2D::LEFT => Direction2D::RIGHT,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Delta2D, Direction2D, Location2D};

    #[test]
    fn delta_sanity() {
        let down = Direction2D::DOWN;
        let down_delta = Delta2D::from(down);
        let down_position = Location2D::try_from(down_delta).unwrap();
        let zero = Location2D { x: 0, y: 0 };
        let reconstructed_delta = zero.delta(down_position).unwrap();
        assert_eq!(down_delta, reconstructed_delta);
    }
}
