use crate::{
    algos::{HuntAndKill, Prim, RecursiveBacktracking},
    grid::Grid,
    utils::types::Coords,
};

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

pub struct MazeSnapshot {
    grid: Grid,
    highlights: Vec<Coords>,
}

impl MazeSnapshot {
    pub fn new(grid: Grid, highlights: Vec<Coords>) -> Self {
        Self { grid, highlights }
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }

    pub fn get_highlights(&self) -> &Vec<Coords> {
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

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub enhanced_graphics: bool,
    pub is_generator_running: bool,
    pub generator: Option<Box<dyn Iterator<Item = MazeSnapshot>>>,
    pub snapshot: Option<MazeSnapshot>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Tab1", "Tab2"]),
            enhanced_graphics,
            is_generator_running: false,
            generator: None,
            snapshot: None,
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'r' => {
                self.run_maze_generator();
            }
            _ => {}
        }
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_tick(&mut self) {
        if let Some(snapshot) = self.get_next_snapshot() {
            self.snapshot = Some(snapshot);
        } else {
            self.is_generator_running = false;
        }
    }

    pub fn run_maze_generator(&mut self) {
        if self.is_generator_running {
            return;
        }

        // let mut method = RecursiveBacktracking::new(20, 20);
        // let mut method = HuntAndKill::new(20, 20);
        let mut method = Prim::new(20, 20);
        method.run();
        self.generator = Some(Box::new(method.into_iter()));

        self.is_generator_running = true;
    }

    fn get_next_snapshot(&mut self) -> Option<MazeSnapshot> {
        if let Some(generator) = &mut self.generator {
            generator.next()
        } else {
            None
        }
    }
}
