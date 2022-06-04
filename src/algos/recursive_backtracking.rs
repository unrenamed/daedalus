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

pub struct RecursiveBacktracking {
    grid: Grid,
    highlights: Vec<Coords>,
    snapshots: Vec<MazeSnapshot>,
}

impl RecursiveBacktracking {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            highlights: vec![],
            snapshots: vec![],
        }
    }

    pub fn run(&mut self) {
        self.highlights.push((0, 0));
        self.carve_passages_from((0, 0));
        self.highlights.pop();
        self.make_snapshot();
    }

    fn carve_passages_from(&mut self, coords: Coords) {
        let mut dirs = [N, W, E, S];
        dirs.shuffle(&mut rand::thread_rng());

        for dir in dirs {
            let next = match self.grid.get_next_cell_coords(coords, dir) {
                Ok(next) => next,
                Err(_) => continue,
            };

            self.make_snapshot();

            if self.grid.is_cell_visited(next) {
                continue;
            }

            if let Ok(next) = self.grid.carve_passage(coords, dir) {
                self.highlights.push(next);
                self.carve_passages_from(next);
                self.highlights.pop();
            }
        }
    }

    fn make_snapshot(&mut self) {
        self.snapshots.push(MazeSnapshot::new(
            self.grid.clone(),
            self.highlights.clone(),
        ));
    }
}

impl IntoIterator for RecursiveBacktracking {
    type Item = MazeSnapshot;
    type IntoIter = RecursiveBacktrackingIter;

    fn into_iter(self) -> Self::IntoIter {
        RecursiveBacktrackingIter {
            snapshots: self.snapshots,
            index: 0,
        }
    }
}

pub struct RecursiveBacktrackingIter {
    snapshots: Vec<MazeSnapshot>,
    index: usize,
}

impl Iterator for RecursiveBacktrackingIter {
    type Item = MazeSnapshot;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.snapshots.len() {
            return None;
        }

        self.index += 1;
        Some(self.snapshots[self.index - 1].clone())
    }
}
