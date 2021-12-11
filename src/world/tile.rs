use std::cmp::Ordering;

pub use super::{
	pixel_pos::PixelPos,
	tile_pos::TilePos,
};

pub const TILE_SIZE: i64 = 128; // Pixels

#[derive(Debug, Clone, PartialEq)]
pub struct Direction {
	pub x: Ordering,
	pub y: Ordering,
}
impl From<TilePos> for Direction {
	fn from(from: TilePos) -> Self {
		let TilePos { x, y } = from;
		let (x, y) = (x.as_ordering(), y.as_ordering());
		Self { x, y }
	}
}

pub trait Asi64 {
	fn as_i64(self) -> i64;
}
impl Asi64 for Ordering {
	fn as_i64(self) -> i64 {
		match self {
			Ordering::Greater => 1,
			Ordering::Equal => 0,
			Ordering::Less => -1,
		}
	}
}
pub trait AsOrdering {
	fn as_ordering(self) -> Ordering;
}
impl AsOrdering for i64 {
	fn as_ordering(self) -> Ordering {
		0.cmp(&self)
	}
}
