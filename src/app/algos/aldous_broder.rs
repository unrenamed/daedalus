use crate::app::{grid::cell::Cell, state::MazeSnapshot, utils::random::get_start_pos};
use rand::prelude::*;

use super::{Generator, IGenerator, Snapshot};

pub struct AldousBroder {
    generator: Generator,
}

impl IGenerator for AldousBroder {
    fn init(width: usize, height: usize) -> Self {
        let generator = Generator::new(width, height);
        Self { generator }
    }

    fn run(&mut self) -> Vec<MazeSnapshot> {
        let width = self.generator.grid.width();
        let height = self.generator.grid.height();

        let (mut x, mut y) = get_start_pos(width, height);

        // The number of remaining unvisited cells
        let mut remaining = width * height - 1;
        while remaining > 0 {
            self.generator.highlights.clear();
            self.generator.highlights.push((x, y));
            self.generator.make_snapshot();

            let mut directions = [Cell::NORTH, Cell::SOUTH, Cell::WEST, Cell::EAST];
            directions.shuffle(&mut rand::thread_rng());

            for dir in directions {
                if let Ok((nx, ny)) = self.generator.grid.get_next_cell_pos((x, y), dir) {
                    if !self.generator.grid.is_cell_visited((nx, ny)) {
                        self.generator.grid.carve_passage((x, y), dir).unwrap();
                        remaining -= 1;
                    }
                    x = nx;
                    y = ny;
                    break;
                }
            }
        }

        self.generator.highlights.clear();
        self.generator.make_snapshot();

        self.generator.get_snapshots()
    }
}
