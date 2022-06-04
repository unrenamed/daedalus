use crate::grid::pole::Pole;
use crate::utils::types::Coords;
use crate::{
    app::MazeSnapshot,
    grid::{
        pole::Pole::{E, N, S, W},
        Grid,
    },
};
use rand::prelude::*;

use super::MazeGenerator;

pub struct Prim {
    grid: Grid,
    highlights: Vec<Coords>,
    snapshots: Vec<MazeSnapshot>,
    frontiers: Vec<Coords>,
}

impl Prim {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            highlights: vec![],
            snapshots: vec![],
            frontiers: vec![],
        }
    }

    pub fn run(&mut self) {
        let mut rng = rand::thread_rng();

        self.mark(get_start_coords(&self.grid));

        self.make_snapshot();

        while !self.frontiers.is_empty() {
            let index = rng.gen_range(0..self.frontiers.len());
            let coords = self.frontiers.remove(index);

            let neighbours = self.neighbours(coords);

            let index = rng.gen_range(0..neighbours.len());
            let (nx, ny) = neighbours[index];

            let (x, y) = coords;

            if let Some(dir) = direction(x, y, nx, ny) {
                self.grid.carve_passage(coords, dir).unwrap();
                self.mark(coords);
            }

            self.make_snapshot();
        }
    }

    fn mark(&mut self, coords: Coords) {
        self.grid.mark_cell(coords);

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
        if x < self.grid.width() && y < self.grid.height() && !self.grid.is_cell_marked((x, y)) {
            if let None = self.frontiers.iter().position(|f| *f == (x, y)) {
                self.frontiers.push((x, y));
            }
        }
    }

    fn neighbours(&self, (x, y): Coords) -> Vec<Coords> {
        let mut neighbours = vec![];

        if x > 0 && self.grid.is_cell_marked((x - 1, y)) {
            neighbours.push((x - 1, y))
        }

        if x + 1 < self.grid.width() && self.grid.is_cell_marked((x + 1, y)) {
            neighbours.push((x + 1, y))
        }

        if y > 0 && self.grid.is_cell_marked((x, y - 1)) {
            neighbours.push((x, y - 1))
        }

        if y + 1 < self.grid.height() && self.grid.is_cell_marked((x, y + 1)) {
            neighbours.push((x, y + 1))
        }

        neighbours
    }

    fn make_snapshot(&mut self) {
        self.highlights.clear();
        self.highlights.extend(&self.frontiers);

        self.snapshots.push(MazeSnapshot::new(
            self.grid.clone(),
            self.highlights.clone(),
        ));
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

impl IntoIterator for Prim {
    type Item = MazeSnapshot;
    type IntoIter = PrimIter;

    fn into_iter(self) -> Self::IntoIter {
        PrimIter {
            snapshots: self.snapshots,
            index: 0,
        }
    }
}

pub struct PrimIter {
    snapshots: Vec<MazeSnapshot>,
    index: usize,
}

impl Iterator for PrimIter {
    type Item = MazeSnapshot;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.snapshots.len() {
            return None;
        }

        self.index += 1;
        Some(self.snapshots[self.index - 1].clone())
    }
}
