use tui::widgets::ListState;

use crate::app::{
    algos::{HuntAndKill, IGenerator, Kruskal, Prim, RecursiveBacktracking},
    grid::Grid,
    utils::types::Pos,
};

pub enum Algorithm {
    RecursiveBacktracking,
    Prims,
    HuntAndKill,
    Kruskal,
}

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

// todo: rename struct and fields
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
    pub tabs: TabsState<'a>,
    pub is_generator_running: bool,
    pub items: StatefulList<(&'a str, Algorithm)>,
    pub snapshots: Option<Vec<MazeSnapshot>>,
    pub curr_idx: usize,
    pub running_algo_idx: Option<usize>,
}

impl<'a> Default for AppState<'a> {
    fn default() -> Self {
        AppState {
            tabs: TabsState::new(vec!["Tab1", "Tab2"]),
            is_generator_running: false,
            snapshots: None,
            curr_idx: 0,
            running_algo_idx: None,
            items: StatefulList::with_items(vec![
                ("Recursive Backtracker", Algorithm::RecursiveBacktracking),
                ("Prim's", Algorithm::Prims),
                ("Hunt & Kill", Algorithm::HuntAndKill),
                ("Kruskal's", Algorithm::Kruskal),
            ]),
        }
    }
}

impl<'a> AppState<'a> {
    pub fn new() -> AppState<'a> {
        AppState {
            ..Default::default()
        }
    }

    pub fn goto_next_tab(&mut self) {
        self.tabs.next();
    }

    pub fn goto_prev_tab(&mut self) {
        self.tabs.previous();
    }

    pub fn select_prev_algo(&mut self) {
        self.items.previous();
    }

    pub fn select_next_algo(&mut self) {
        self.items.next();
    }

    pub fn on_tick(&mut self) {
        if self.items.state.selected().is_none() {
            self.items.next();
        }

        if !self.is_generator_running {
            return;
        }

        if self.get_next_snapshot().is_some() {
            self.curr_idx += 1;
        } else {
            self.is_generator_running = false;
        }
    }

    pub fn start_maze_generation(&mut self) {
        if self.is_generator_running {
            return;
        }

        if let Some(idx) = self.items.state.selected() {
            if let Some(algo) = self.items.items.get(idx) {
                let snapshots = match algo.1 {
                    Algorithm::Prims => Prim::init(20, 20).run(),
                    Algorithm::RecursiveBacktracking => RecursiveBacktracking::init(20, 20).run(),
                    Algorithm::HuntAndKill => HuntAndKill::init(20, 20).run(),
                    Algorithm::Kruskal => Kruskal::init(20, 20).run(),
                };

                self.snapshots = Some(snapshots);
                self.curr_idx = 0;
                self.running_algo_idx = Some(idx);
                self.is_generator_running = true;
            }
        }
    }

    pub fn get_next_snapshot(&self) -> Option<&MazeSnapshot> {
        self.get_snapshot(self.curr_idx + 1)
    }

    pub fn get_curr_snapshot(&self) -> Option<&MazeSnapshot> {
        self.get_snapshot(self.curr_idx)
    }

    pub fn get_running_algorithm_title(&self) -> Option<&str> {
        if let Some(idx) = self.running_algo_idx {
            if let Some(algo) = self.items.items.get(idx) {
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
}
