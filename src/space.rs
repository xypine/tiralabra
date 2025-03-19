use std::{num::TryFromIntError, ops::Add};

use crate::interface::{Direction, Location};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Vector2D<T: Copy> {
    pub x: T,
    pub y: T,
}

pub type Location2D = Vector2D<usize>;
impl Location2D {
    pub fn try_apply(self, delta: Delta2D) -> Result<Self, TryFromIntError> {
        let self_as_delta = Delta2D::try_from(self)?;
        let result = self_as_delta + delta;
        Self::try_from(result)
    }
}

impl Location<2> for Location2D {}

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

#[derive(Debug, Clone, Copy, strum_macros::EnumIter)]
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

impl Direction<4> for Direction2D {}
