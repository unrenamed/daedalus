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

pub struct HuntAndKill {
    grid: Grid,
    highlights: Vec<Coords>,
    snapshots: Vec<MazeSnapshot>,
    hunt_start_index: usize,
    highlighted_count: usize,
}

impl HuntAndKill {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            highlights: vec![],
            snapshots: vec![],
            hunt_start_index: 0,
            highlighted_count: 0,
        }
    }

    fn walk(&mut self, coords: Coords) -> Option<Coords> {
        let mut directions = [N, E, W, S];
        directions.shuffle(&mut rand::thread_rng());

        for dir in directions {
            if let Ok(next_coords) = self.grid.get_next_cell_coords(coords, dir) {
                if !self.grid.is_cell_visited(next_coords) {
                    return self.grid.carve_passage(coords, dir).ok();
                }
            }
        }

        None
    }

    fn hunt(&mut self) -> Option<Coords> {
        let directions = [N, E, W, S];

        for y in self.hunt_start_index..self.grid.height() {
            for x in 0..self.grid.width() {
                self.highlights.push((x, y));
            }

            self.make_snapshot();

            for _ in 0..self.grid.width() {
                self.highlights.pop();
            }

            let mut unvisited_cells_count = 0;

            for x in 0..self.grid.width() {
                if self.grid.is_cell_visited((x, y)) {
                    continue;
                } else {
                    unvisited_cells_count += 1;
                }

                for dir in directions {
                    if let Ok(next_coords) = self.grid.get_next_cell_coords((x, y), dir) {
                        if self.grid.is_cell_visited(next_coords) {
                            self.grid.carve_passage((x, y), dir).ok();

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

    pub fn run(&mut self) {
        let start_coords = get_start_coords(&self.grid);
        let mut x = start_coords.0;
        let mut y = start_coords.1;

        loop {
            self.make_snapshot();

            if let Some((nx, ny)) = self.walk((x, y)) {
                x = nx;
                y = ny;
                self.highlights.push((x, y));
                self.highlighted_count += 1;
            } else if let Some((nx, ny)) = self.hunt() {
                for _ in 0..self.highlighted_count {
                    self.highlights.pop();
                }
                self.highlighted_count = 0;

                x = nx;
                y = ny;
            } else {
                for _ in 0..self.highlighted_count {
                    self.highlights.pop();
                }
                self.highlighted_count = 0;
                break;
            }
        }

        self.make_snapshot();
    }

    fn make_snapshot(&mut self) {
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

impl IntoIterator for HuntAndKill {
    type Item = MazeSnapshot;
    type IntoIter = HuntAndKillIter;

    fn into_iter(self) -> Self::IntoIter {
        HuntAndKillIter {
            snapshots: self.snapshots,
            index: 0,
        }
    }
}

pub struct HuntAndKillIter {
    snapshots: Vec<MazeSnapshot>,
    index: usize,
}

impl Iterator for HuntAndKillIter {
    type Item = MazeSnapshot;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.snapshots.len() {
            return None;
        }

        self.index += 1;
        Some(self.snapshots[self.index - 1].clone())
    }
}
