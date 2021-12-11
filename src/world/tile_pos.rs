use std::ops::*;

use crow::glutin::dpi::PhysicalSize;

use crate::TILE_SIZE;

use super::tile::{
	Asi64,
	Direction,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TilePos {
	pub x: i64,
	pub y: i64,
}
impl From<(i64, i64)> for TilePos {
	fn from(from: (i64, i64)) -> Self {
		let (x, y) = from;
		Self { x, y }
	}
}
impl From<Direction> for TilePos {
	fn from(from: Direction) -> Self {
		let Direction { x, y } = from;
		let (x, y) = (x.as_i64(), y.as_i64());
		Self { x, y }
	}
}
impl From<PhysicalSize<u32>> for TilePos {
	fn from(from: PhysicalSize<u32>) -> Self {
		let (x, y) = (
			i64::from(from.width) / TILE_SIZE,
			i64::from(from.height) / TILE_SIZE,
		);
		Self { x, y }
	}
}
impl Add<&TilePos> for TilePos {
	type Output = Self;
	fn add(self, rhs: &Self) -> Self {
		Self {
			x: self.x.add(rhs.x),
			y: self.y.add(rhs.y),
		}
	}
}
impl Sub<&TilePos> for TilePos {
	type Output = Self;
	fn sub(self, rhs: &Self) -> Self {
		Self {
			x: self.x.sub(rhs.x),
			y: self.y.sub(rhs.y),
		}
	}
}
impl Mul<&TilePos> for TilePos {
	type Output = Self;
	fn mul(self, rhs: &Self) -> Self {
		Self {
			x: self.x.mul(rhs.x),
			y: self.y.mul(rhs.y),
		}
	}
}
impl Div<&TilePos> for TilePos {
	type Output = Self;
	fn div(self, rhs: &Self) -> Self {
		Self {
			x: self.x.div(rhs.x),
			y: self.y.div(rhs.y),
		}
	}
}
impl AddAssign<&TilePos> for TilePos {
	fn add_assign(&mut self, rhs: &Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}
impl TilePos {
	pub fn to_world_tile(self, center_world_pos: &TilePos) -> TilePos {
		self + center_world_pos
	}
}
