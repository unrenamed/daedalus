use crate::app::{state::MazeSnapshot, utils::types::Pos, grid::cell::Cell};
use rand::prelude::*;

use super::{Generator, IGenerator, Snapshot};

pub struct RecursiveBacktracking {
    generator: Generator,
}

impl RecursiveBacktracking {
    fn carve_passages_from(&mut self, pos: Pos) {
        let mut dirs = [Cell::NORTH, Cell::SOUTH, Cell::WEST, Cell::EAST];
        dirs.shuffle(&mut rand::thread_rng());

        for dir in dirs {
            let next = match self.generator.grid.get_next_cell_pos(pos, dir) {
                Ok(next) => next,
                Err(_) => continue,
            };

            self.generator.make_snapshot();

            if self.generator.grid.is_cell_visited(next) {
                continue;
            }

            if let Ok(next) = self.generator.grid.carve_passage(pos, dir) {
                self.generator.highlights.push(next);
                self.carve_passages_from(next);
                self.generator.highlights.pop();
            }
        }
    }
}

impl IGenerator for RecursiveBacktracking {
    fn init(width: usize, height: usize) -> Self {
        Self {
            generator: Generator::new(width, height),
        }
    }

    fn run(&mut self) -> Vec<MazeSnapshot> {
        self.generator.highlights.push((0, 0));
        self.carve_passages_from((0, 0));
        self.generator.highlights.pop();
        self.generator.make_snapshot();

        self.generator.get_snapshots()
    }
}
