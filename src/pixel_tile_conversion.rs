use crate::TILE_SIZE;
pub fn pixel_to_tile_pos(
	window_size: (i64, i64),
	pixel_pos: (i64, i64),
	player_pos: (i64, i64),
) -> (i64, i64) {
	/*let offset = (
		(window_size.0 - ((window_size.0 / TILE_SIZE) * TILE_SIZE)) / 2,
		(window_size.1 - ((window_size.1 / TILE_SIZE) * TILE_SIZE)) / 2,
	);*/
	let centered_pixel_pos = (
		(pixel_pos.0 as f64 - (window_size.0 as f64 / 2.0)),
		(pixel_pos.1 as f64 - (window_size.1 as f64 / 2.0)),
	);
	let player_dist = (
		centered_pixel_pos.0 / TILE_SIZE as f64,
		centered_pixel_pos.1 / TILE_SIZE as f64,
	);

	let tile_pos = (
		player_pos.0 as f64 + (player_dist.0 / 2.0), // Needs to be halved for some weird reason. TODO: Figure out
		player_pos.1 as f64 + (player_dist.1 / 2.0),
	);

	(tile_pos.0 as i64, tile_pos.1 as i64)
}
