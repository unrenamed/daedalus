use crate::grid::pole::Pole;
use crate::utils::types::Coords;
use crate::{app::MazeSnapshot, grid::Grid};
use rand::prelude::*;

use super::{Generator, IGenerator, Snapshot};

pub struct Prim {
    generator: Generator,
    frontiers: Vec<Coords>,
}

impl Prim {
    fn mark(&mut self, coords: Coords) {
        self.generator.grid.mark_cell(coords);

        let (x, y) = coords;
        self.add_frontier((x + 1, y));
        self.add_frontier((x, y + 1));
        if x > 0 {
            self.add_frontier((x - 1, y));
        }
        if y > 0 {
            self.add_frontier((x, y - 1));
        }
    }

    fn add_frontier(&mut self, (x, y): Coords) {
        if x < self.generator.grid.width()
            && y < self.generator.grid.height()
            && !self.generator.grid.is_cell_marked((x, y))
        {
            if let None = self.frontiers.iter().position(|f| *f == (x, y)) {
                self.frontiers.push((x, y));
            }
        }
    }

    fn neighbours(&self, (x, y): Coords) -> Vec<Coords> {
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

        self.mark(get_start_coords(&self.generator.grid));

        self.generator.highlights.clear();
        self.generator.highlights.extend(&self.frontiers);
        self.generator.make_snapshot();

        while !self.frontiers.is_empty() {
            let index = rng.gen_range(0..self.frontiers.len());
            let coords = self.frontiers.remove(index);

            let neighbours = self.neighbours(coords);

            let index = rng.gen_range(0..neighbours.len());
            let (nx, ny) = neighbours[index];

            let (x, y) = coords;

            if let Some(dir) = direction(x, y, nx, ny) {
                self.generator.grid.carve_passage(coords, dir).unwrap();
                self.mark(coords);
            }

            self.generator.highlights.clear();
            self.generator.highlights.extend(&self.frontiers);
            self.generator.make_snapshot();
        }

        self.generator.get_snapshots()
    }
}

fn get_start_coords(grid: &Grid) -> Coords {
    let mut rng = rand::thread_rng();
    let y = rng.gen_range(0..grid.height());
    let x = rng.gen_range(0..grid.width());
    (x, y)
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
