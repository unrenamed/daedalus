use crate::{app::MazeSnapshot, grid::Grid, utils::types::Coords};

pub mod hunt_and_kill;
pub mod prim;
pub mod recursive_backtracking;

pub use hunt_and_kill::HuntAndKill;
pub use prim::Prim;
pub use recursive_backtracking::RecursiveBacktracking;

pub trait Snapshot {
    fn make_snapshot(&mut self);
    fn get_snapshots(&self) -> Vec<MazeSnapshot>;
}

pub struct Generator {
    grid: Grid,
    highlights: Vec<Coords>,
    snapshots: Vec<MazeSnapshot>,
}

impl Generator {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            highlights: vec![],
            snapshots: vec![],
        }
    }
}

impl Snapshot for Generator {
    fn make_snapshot(&mut self) {
        self.snapshots.push(MazeSnapshot::new(
            self.grid.clone(),
            self.highlights.clone(),
        ));
    }

    fn get_snapshots(&self) -> Vec<MazeSnapshot> {
        self.snapshots.clone()
    }
}

pub trait IGenerator {
    fn init(width: usize, height: usize) -> Self;
    fn run(&mut self) -> Vec<MazeSnapshot>;
}
