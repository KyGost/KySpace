use crow::Texture;

pub trait Tile {
	fn player_can_walk() -> bool {
		false
	}
	fn sprite_count() -> usize {
		1
	}
	fn get_sprite() -> Texture;
	fn draw_sprite() {}
}
