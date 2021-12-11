use {
	crate::{
		atlas::Atlas,
		world::tile::PixelPos,
		Error,
	},
	crow::{
		Context,
		WindowSurface,
	},
};

pub trait Draw {
	fn draw(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		pos: PixelPos,
		atlas: &Atlas,
		frame: usize,
	) -> Result<(), Error> {
		self.draw_animated(ctx, surface, pos, atlas, frame)
			.or_else(|_| self.draw_still(ctx, surface, pos, atlas))
	}
	fn draw_animated(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		pos: PixelPos,
		atlas: &Atlas,
		frame: usize,
	) -> Result<(), Error> {
		Err(Error::AnimationUnimplemented)
	}
	fn draw_still(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		pos: PixelPos,
		atlas: &Atlas,
	) -> Result<(), Error>;
}
