use crate::utils::types::Pos;
use crate::{
    app::MazeSnapshot,
    grid::{
        pole::Pole::{E, N, S, W},
        Grid,
    },
};
use rand::prelude::*;

use super::{Generator, IGenerator, Snapshot};

pub struct HuntAndKill {
    generator: Generator,
    hunt_start_index: usize,
    highlighted_count: usize,
}

impl HuntAndKill {
    fn walk(&mut self, pos: Pos) -> Option<Pos> {
        let mut directions = [N, E, W, S];
        directions.shuffle(&mut rand::thread_rng());

        for dir in directions {
            if let Ok(next_pos) = self.generator.grid.get_next_cell_pos(pos, dir) {
                if !self.generator.grid.is_cell_visited(next_pos) {
                    return self.generator.grid.carve_passage(pos, dir).ok();
                }
            }
        }

        None
    }

    fn hunt(&mut self) -> Option<Pos> {
        let directions = [N, E, W, S];

        for y in self.hunt_start_index..self.generator.grid.height() {
            for x in 0..self.generator.grid.width() {
                self.generator.highlights.push((x, y));
            }

            self.generator.make_snapshot();

            for _ in 0..self.generator.grid.width() {
                self.generator.highlights.pop();
            }

            let mut unvisited_cells_count = 0;

            for x in 0..self.generator.grid.width() {
                if self.generator.grid.is_cell_visited((x, y)) {
                    continue;
                } else {
                    unvisited_cells_count += 1;
                }

                for dir in directions {
                    if let Ok(next_pos) = self.generator.grid.get_next_cell_pos((x, y), dir) {
                        if self.generator.grid.is_cell_visited(next_pos) {
                            self.generator.grid.carve_passage((x, y), dir).ok();

                            return Some((x, y));
                        }
                    }
                }
            }

            if unvisited_cells_count == 0 {
                self.hunt_start_index = y + 1;
            }
        }
        None
    }
}

impl IGenerator for HuntAndKill {
    fn init(width: usize, height: usize) -> Self {
        let generator = Generator::new(width, height);
        Self {
            generator,
            hunt_start_index: 0,
            highlighted_count: 0,
        }
    }

    fn run(&mut self) -> Vec<MazeSnapshot> {
        let start_pos = get_start_pos(&self.generator.grid);
        let mut x = start_pos.0;
        let mut y = start_pos.1;

        loop {
            self.generator.make_snapshot();

            if let Some((nx, ny)) = self.walk((x, y)) {
                x = nx;
                y = ny;
                self.generator.highlights.push((x, y));
                self.highlighted_count += 1;
            } else if let Some((nx, ny)) = self.hunt() {
                for _ in 0..self.highlighted_count {
                    self.generator.highlights.pop();
                }
                self.highlighted_count = 0;

                x = nx;
                y = ny;
            } else {
                for _ in 0..self.highlighted_count {
                    self.generator.highlights.pop();
                }
                self.highlighted_count = 0;
                break;
            }
        }

        self.generator.make_snapshot();

        self.generator.get_snapshots()
    }
}

fn get_start_pos(grid: &Grid) -> Pos {
    let mut rng = rand::thread_rng();
    let y = rng.gen_range(0..grid.height());
    let x = rng.gen_range(0..grid.width());
    (x, y)
}
