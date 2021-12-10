use crate::TILE_SIZE;
pub fn pixel_to_tile_pos(
	window_size: (i64, i64),
	pixel_pos: (i64, i64),
	player_pos: (i64, i64),
) -> (i64, i64) {
	let offset = (
		(window_size.0 - ((window_size.0 / TILE_SIZE) * TILE_SIZE)) / 2,
		(window_size.1 - ((window_size.1 / TILE_SIZE) * TILE_SIZE)) / 2,
	);
	let centered_pixel_pos = (
		(pixel_pos.0 - (window_size.0 / 2)),
		(pixel_pos.1 - (window_size.1 / 2)),
	);
	let player_dist = (
		(centered_pixel_pos.0 - offset.0) / TILE_SIZE,
		(centered_pixel_pos.1 - offset.1) / TILE_SIZE,
	);

	let tile_pos = (player_pos.0 + player_dist.0, player_pos.1 + player_dist.1);

	(tile_pos.0 / 2, tile_pos.1 / 2) // Needs to be halved for some weird reason. TODO: Figure out
}
