use rand::Rng;

use super::types::Pos;

pub fn get_start_pos(max_x: usize, max_y: usize) -> Pos {
    let mut rng = rand::thread_rng();
    let y = rng.gen_range(0..max_y);
    let x = rng.gen_range(0..max_x);
    (x, y)
}
