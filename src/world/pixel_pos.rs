use std::ops::*;

use crow::glutin::dpi::{
	PhysicalPosition,
	PhysicalSize,
};

use crate::TILE_SIZE;

use super::tile_pos::TilePos;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PixelPos {
	pub x: i64,
	pub y: i64,
}
impl From<(i64, i64)> for PixelPos {
	fn from(from: (i64, i64)) -> Self {
		let (x, y) = from;
		Self { x, y }
	}
}
impl From<TilePos> for PixelPos {
	fn from(from: TilePos) -> Self {
		let TilePos { x, y } = from * &(TILE_SIZE, TILE_SIZE).into();
		Self { x, y }
	}
}
impl From<PhysicalSize<u32>> for PixelPos {
	fn from(from: PhysicalSize<u32>) -> Self {
		Self {
			x: from.width.into(),
			y: from.height.into(),
		}
	}
}
impl Into<(i64, i64)> for PixelPos {
	fn into(self) -> (i64, i64) {
		(self.x, self.y)
	}
}
impl Into<(i32, i32)> for PixelPos {
	fn into(self) -> (i32, i32) {
		(self.x.try_into().unwrap(), self.y.try_into().unwrap()) // TODO: Handle
	}
}
impl Add<&PixelPos> for PixelPos {
	type Output = Self;
	fn add(self, rhs: &Self) -> Self {
		Self {
			x: self.x.add(rhs.x),
			y: self.y.add(rhs.y),
		}
	}
}
impl Sub<&PixelPos> for PixelPos {
	type Output = Self;
	fn sub(self, rhs: &Self) -> Self {
		Self {
			x: self.x.sub(rhs.x),
			y: self.y.sub(rhs.y),
		}
	}
}
impl Mul<&PixelPos> for PixelPos {
	type Output = Self;
	fn mul(self, rhs: &Self) -> Self {
		Self {
			x: self.x.mul(rhs.x),
			y: self.y.mul(rhs.y),
		}
	}
}
impl Div<&PixelPos> for PixelPos {
	type Output = Self;
	fn div(self, rhs: &Self) -> Self {
		Self {
			x: self.x.div(rhs.x),
			y: self.y.div(rhs.y),
		}
	}
}
impl PixelPos {
	pub fn from_mouse(from: PhysicalPosition<f64>, window_size: PhysicalSize<u32>) -> Self {
		let PhysicalPosition { x, y } = from;
		let (x, y) = (x as i64, y as i64);
		let y = i64::from(window_size.height) - y; // Mouse Y is inverse to Display Y
		Self { x, y }
	}
	pub fn to_rel_tile_pos(self, window_size: PhysicalSize<u32>) -> TilePos {
		let Self { x, y } = self;
		let (x, y) = (
			self.x - (i64::from(window_size.width) / 2),
			self.y - (i64::from(window_size.height) / 2),
		); // centered
		let (x, y) = (x / TILE_SIZE, y / TILE_SIZE);
		TilePos { x, y }
	}
}
