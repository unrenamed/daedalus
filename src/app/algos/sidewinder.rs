use crate::app::{state::MazeSnapshot, grid::cell::Cell};
use rand::prelude::*;

use super::{Generator, IGenerator, Snapshot};

pub struct Sidewinder {
    generator: Generator,
}

impl Sidewinder {
    fn generate(&mut self) {
        let mut rng = rand::thread_rng();

        for y in 0..self.generator.grid.height() {
            let mut run_start = 0;

            for x in 0..self.generator.grid.width() {
                let carve_east: bool = rng.gen();

                if y == 0 || (carve_east && x + 1 < self.generator.grid.width()) {
                    self.generator.grid.carve_passage((x, y), Cell::EAST).ok();
                } else {
                    let rand_x = rng.gen_range(run_start..=x);
                    self.generator.grid.carve_passage((rand_x, y), Cell::NORTH).ok();
                    run_start = x + 1;
                }

                self.generator.make_snapshot();
            }
        }
    }
}

impl IGenerator for Sidewinder {
    fn init(width: usize, height: usize) -> Self {
        Self {
            generator: Generator::new(width, height),
        }
    }

    fn run(&mut self) -> Vec<MazeSnapshot> {
        self.generate();
        self.generator.make_snapshot();
        self.generator.get_snapshots()
    }
}
