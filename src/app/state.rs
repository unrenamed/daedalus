use tui::widgets::ListState;

use crate::app::{
    algos::{AldousBroder, Eller, HuntAndKill, IGenerator, Kruskal, Prim, RecursiveBacktracking, Sidewinder},
    grid::Grid,
    utils::types::Pos,
};

pub enum Algorithm {
    RecursiveBacktracking,
    Prims,
    HuntAndKill,
    Kruskal,
    AldousBroder,
    Eller,
    Sidewinder,
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct MazeSnapshot {
    grid: Grid,
    highlights: Vec<Pos>,
}

impl MazeSnapshot {
    pub fn new(grid: Grid, highlights: Vec<Pos>) -> Self {
        Self { grid, highlights }
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }

    pub fn get_highlights(&self) -> &Vec<Pos> {
        &self.highlights
    }
}

impl Clone for MazeSnapshot {
    fn clone(&self) -> Self {
        Self {
            grid: self.grid.clone(),
            highlights: self.highlights.clone(),
        }
    }
}

pub struct AppState<'a> {
    pub grid_width: usize,
    pub grid_height: usize,
    pub is_generator_running: bool,
    pub algorithms: StatefulList<(&'a str, Algorithm)>,
    pub snapshots: Option<Vec<MazeSnapshot>>,
    pub curr_algo_idx: usize,
    pub running_algo_idx: Option<usize>,
}

impl<'a> Default for AppState<'a> {
    fn default() -> Self {
        AppState {
            grid_width: 10,
            grid_height: 10,
            is_generator_running: false,
            snapshots: None,
            curr_algo_idx: 0,
            running_algo_idx: None,
            algorithms: StatefulList::with_items(vec![
                ("Recursive Backtracker", Algorithm::RecursiveBacktracking),
                ("Prim's", Algorithm::Prims),
                ("Hunt & Kill", Algorithm::HuntAndKill),
                ("Kruskal's", Algorithm::Kruskal),
                ("Aldou-Broder's", Algorithm::AldousBroder),
                ("Eller's", Algorithm::Eller),
                ("Sidewinder", Algorithm::Sidewinder),
            ]),
        }
    }
}

impl<'a> AppState<'a> {
    pub fn new(grid_width: usize, grid_height: usize) -> AppState<'a> {
        AppState {
            grid_width,
            grid_height,
            ..Default::default()
        }
    }

    pub fn select_prev_algo(&mut self) {
        if !self.is_generator_running {
            self.algorithms.previous();
        }
    }

    pub fn select_next_algo(&mut self) {
        if !self.is_generator_running {
            self.algorithms.next();
        }
    }

    pub fn on_tick(&mut self) {
        if self.algorithms.state.selected().is_none() {
            self.algorithms.next();
        }

        if !self.is_generator_running {
            return;
        }

        if self.get_next_snapshot().is_some() {
            self.curr_algo_idx += 1;
        } else {
            self.is_generator_running = false;
        }
    }

    pub fn start_maze_generation(&mut self) {
        if self.is_generator_running {
            return;
        }

        if let Some(idx) = self.algorithms.state.selected() {
            if let Some(algo) = self.algorithms.items.get(idx) {
                let snapshots = match algo.1 {
                    Algorithm::Prims => self.generate_maze::<Prim>(),
                    Algorithm::RecursiveBacktracking => self.generate_maze::<RecursiveBacktracking>(),
                    Algorithm::HuntAndKill => self.generate_maze::<HuntAndKill>(),
                    Algorithm::Kruskal => self.generate_maze::<Kruskal>(),
                    Algorithm::AldousBroder => self.generate_maze::<AldousBroder>(),
                    Algorithm::Eller => self.generate_maze::<Eller>(),
                    Algorithm::Sidewinder => self.generate_maze::<Sidewinder>(),
                };

                self.snapshots = Some(snapshots);
                self.curr_algo_idx = 0;
                self.running_algo_idx = Some(idx);
                self.is_generator_running = true;
            }
        }
    }

    pub fn get_next_snapshot(&self) -> Option<&MazeSnapshot> {
        self.get_snapshot(self.curr_algo_idx + 1)
    }

    pub fn get_curr_snapshot(&self) -> Option<&MazeSnapshot> {
        self.get_snapshot(self.curr_algo_idx)
    }

    pub fn get_running_algorithm_title(&self) -> Option<&str> {
        if let Some(idx) = self.running_algo_idx {
            if let Some(algo) = self.algorithms.items.get(idx) {
                return Some(algo.0);
            }
        }
        None
    }

    fn get_snapshot(&self, idx: usize) -> Option<&MazeSnapshot> {
        if let Some(snapshosts) = &self.snapshots {
            snapshosts.get(idx)
        } else {
            None
        }
    }

    fn generate_maze<T: IGenerator>(&self) -> Vec<MazeSnapshot> {
        T::init(self.grid_width, self.grid_height).run()
    }
}
