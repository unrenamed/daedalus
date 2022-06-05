use crate::app::MazeSnapshot;
use crate::grid::pole::Pole;
use crate::utils::random::get_start_pos;
use crate::utils::types::Pos;
use rand::prelude::*;

use super::{Generator, IGenerator, Snapshot};

pub struct Prim {
    generator: Generator,
    frontiers: Vec<Pos>,
}

impl Prim {
    fn mark(&mut self, pos: Pos) {
        self.generator.grid.mark_cell(pos);

        let (x, y) = pos;
        self.add_frontier((x + 1, y));
        self.add_frontier((x, y + 1));
        if x > 0 {
            self.add_frontier((x - 1, y));
        }
        if y > 0 {
            self.add_frontier((x, y - 1));
        }
    }

    fn add_frontier(&mut self, (x, y): Pos) {
        if x < self.generator.grid.width()
            && y < self.generator.grid.height()
            && !self.generator.grid.is_cell_marked((x, y))
        {
            if let None = self.frontiers.iter().position(|f| *f == (x, y)) {
                self.frontiers.push((x, y));
            }
        }
    }

    fn neighbours(&self, (x, y): Pos) -> Vec<Pos> {
        let mut neighbours = vec![];

        if x > 0 && self.generator.grid.is_cell_marked((x - 1, y)) {
            neighbours.push((x - 1, y))
        }

        if x + 1 < self.generator.grid.width() && self.generator.grid.is_cell_marked((x + 1, y)) {
            neighbours.push((x + 1, y))
        }

        if y > 0 && self.generator.grid.is_cell_marked((x, y - 1)) {
            neighbours.push((x, y - 1))
        }

        if y + 1 < self.generator.grid.height() && self.generator.grid.is_cell_marked((x, y + 1)) {
            neighbours.push((x, y + 1))
        }

        neighbours
    }
}

impl IGenerator for Prim {
    fn init(width: usize, height: usize) -> Self {
        let generator = Generator::new(width, height);
        Self {
            generator,
            frontiers: vec![],
        }
    }

    fn run(&mut self) -> Vec<MazeSnapshot> {
        let mut rng = rand::thread_rng();

        let start_pos = get_start_pos(self.generator.grid.width(), self.generator.grid.height());
        self.mark(start_pos);

        self.generator.highlights.clear();
        self.generator.highlights.extend(&self.frontiers);
        self.generator.make_snapshot();

        while !self.frontiers.is_empty() {
            let index = rng.gen_range(0..self.frontiers.len());
            let pos = self.frontiers.remove(index);

            let neighbours = self.neighbours(pos);

            let index = rng.gen_range(0..neighbours.len());
            let (nx, ny) = neighbours[index];

            let (x, y) = pos;

            if let Some(dir) = direction(x, y, nx, ny) {
                self.generator.grid.carve_passage(pos, dir).unwrap();
                self.mark(pos);
            }

            self.generator.highlights.clear();
            self.generator.highlights.extend(&self.frontiers);
            self.generator.make_snapshot();
        }

        self.generator.get_snapshots()
    }
}

fn direction(x: usize, y: usize, nx: usize, ny: usize) -> Option<Pole> {
    if x < nx {
        return Some(Pole::E);
    }
    if x > nx {
        return Some(Pole::W);
    }
    if y < ny {
        return Some(Pole::S);
    }
    if y > ny {
        return Some(Pole::N);
    }

    None
}
