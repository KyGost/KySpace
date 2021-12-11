use crow::{
	Context,
	WindowSurface,
};

use crate::{
	atlas::Atlas,
	world::tile::PixelPos,
};

pub trait Draw {
	fn draw(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		position: PixelPos,
		atlas: &Atlas,
	);
}
