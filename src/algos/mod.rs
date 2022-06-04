use crate::grid::Grid;

pub mod hunt_and_kill;
pub mod prim;
pub mod recursive_backtracking;

pub use hunt_and_kill::HuntAndKill;
pub use prim::Prim;
pub use recursive_backtracking::RecursiveBacktracking;

pub trait MazeGenerator {
    fn generate(&mut self, grid: &mut Grid) -> Self;
}
