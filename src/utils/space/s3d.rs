//! Three dimensional space

use std::{
    hash::Hash,
    num::TryFromIntError,
    ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use tsify_next::declare;

use super::{Direction, Location};

pub const AXIS_3D: usize = 3;
pub const NEIGHBOUR_COUNT_3D: usize = 2 * AXIS_3D;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Tsify, Serialize, Deserialize)]
pub struct Vector3D<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[declare]
pub type Location3D = Vector3D<usize>;
impl Location3D {
    pub fn try_apply(self, delta: Delta3D) -> Result<Self, TryFromIntError> {
        let self_as_delta = Delta3D::try_from(self)?;
        let result = self_as_delta + delta;
        Self::try_from(result)
    }

    pub fn delta(self, other: Self) -> Result<Delta3D, TryFromIntError> {
        let self_as_delta = Delta3D::try_from(self)?;
        let other_as_delta = Delta3D::try_from(other)?;
        Ok(other_as_delta - self_as_delta)
    }
}

impl Location for Location3D {
    fn length(&self) -> usize {
        self.x * self.y * self.z
    }
}

impl TryFrom<Location3D> for Delta3D {
    type Error = TryFromIntError;

    fn try_from(value: Location3D) -> Result<Self, Self::Error> {
        let x = isize::try_from(value.x)?;
        let y = isize::try_from(value.y)?;
        let z = isize::try_from(value.z)?;
        Ok(Self { x, y, z })
    }
}
impl TryFrom<Delta3D> for Location3D {
    type Error = TryFromIntError;

    fn try_from(value: Delta3D) -> Result<Self, Self::Error> {
        let x = usize::try_from(value.x)?;
        let y = usize::try_from(value.y)?;
        let z = usize::try_from(value.z)?;
        Ok(Self { x, y, z })
    }
}

pub type Delta3D = Vector3D<isize>;

impl Add<Delta3D> for Delta3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub<Delta3D> for Delta3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Tsify, Serialize, Deserialize,
)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Direction3D {
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
    FORWARDS = 4,
    BACKWARDS = 5,
}

impl TryFrom<usize> for Direction3D {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UP),
            1 => Ok(Self::RIGHT),
            2 => Ok(Self::DOWN),
            3 => Ok(Self::LEFT),
            4 => Ok(Self::FORWARDS),
            5 => Ok(Self::BACKWARDS),
            _ => Err(()),
        }
    }
}

impl From<Direction3D> for Delta3D {
    fn from(value: Direction3D) -> Self {
        match value {
            Direction3D::UP => Delta3D { x: 0, y: -1, z: 0 },
            Direction3D::RIGHT => Delta3D { x: 1, y: 0, z: 0 },
            Direction3D::DOWN => Delta3D { x: 0, y: 1, z: 0 },
            Direction3D::LEFT => Delta3D { x: -1, y: 0, z: 0 },
            Direction3D::FORWARDS => Delta3D { x: 0, y: 0, z: 1 },
            Direction3D::BACKWARDS => Delta3D { x: 0, y: 0, z: -1 },
        }
    }
}

impl TryFrom<Delta3D> for Direction3D {
    type Error = ();

    fn try_from(value: Delta3D) -> Result<Self, Self::Error> {
        match value {
            Delta3D { x: 0, y: -1, z: 0 } => Ok(Direction3D::UP),
            Delta3D { x: 1, y: 0, z: 0 } => Ok(Direction3D::RIGHT),
            Delta3D { x: 0, y: 1, z: 0 } => Ok(Direction3D::DOWN),
            Delta3D { x: -1, y: 0, z: 0 } => Ok(Direction3D::LEFT),
            Delta3D { x: 0, y: 0, z: 1 } => Ok(Direction3D::FORWARDS),
            Delta3D { x: 0, y: 0, z: -1 } => Ok(Direction3D::BACKWARDS),
            _ => Err(()),
        }
    }
}

impl Direction<NEIGHBOUR_COUNT_3D> for Direction3D {
    fn mirror(self) -> Self {
        match self {
            Direction3D::UP => Direction3D::DOWN,
            Direction3D::RIGHT => Direction3D::LEFT,
            Direction3D::DOWN => Direction3D::UP,
            Direction3D::LEFT => Direction3D::RIGHT,
            Direction3D::FORWARDS => Direction3D::BACKWARDS,
            Direction3D::BACKWARDS => Direction3D::FORWARDS,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Delta3D, Direction3D, Location3D};

    #[test]
    fn delta_sanity() {
        let down = Direction3D::DOWN;
        let down_delta = Delta3D::from(down);
        let down_position = Location3D::try_from(down_delta).unwrap();
        let zero = Location3D { x: 0, y: 0, z: 0 };
        let reconstructed_delta = zero.delta(down_position).unwrap();
        assert_eq!(down_delta, reconstructed_delta);
    }
}
