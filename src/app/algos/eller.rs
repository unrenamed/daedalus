use crate::app::{
    grid::pole::Pole::{self},
    state::MazeSnapshot,
    utils::types::Pos,
};
use rand::prelude::*;

use super::{Generator, IGenerator, Snapshot};

use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CellId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SetId(usize);

#[derive(Debug, Clone, Copy)]
struct Cell {
    _id: CellId,
    set_id: SetId,
    pos: Pos,
}

struct State {
    width: usize,
    next_set_id: Option<usize>,
    row_num: usize,
    cells: HashMap<CellId, RefCell<Cell>>,
}

impl State {
    fn new(row_num: usize, next_set_id: Option<usize>, width: usize) -> State {
        State {
            width,
            next_set_id,
            row_num,
            cells: HashMap::new(),
        }
    }

    fn next(&self) -> State {
        State::new(self.row_num + 1, self.next_set_id, self.width)
    }

    fn populate(mut self) -> Self {
        for n in 1..=self.width {
            let cell_id = CellId(n);
            if self.cells.get(&cell_id).is_some() {
                continue;
            }

            self.next_set_id = Some(self.next_set_id.unwrap_or(0) + 1);
            let set_id = SetId(self.next_set_id.unwrap());

            self.add(cell_id, set_id, (n - 1, self.row_num));
        }

        self
    }

    fn add(&mut self, _id: CellId, set_id: SetId, pos: Pos) {
        let cell = Cell { _id, set_id, pos };
        self.cells.insert(_id, RefCell::new(cell));
    }

    fn connect(&mut self, sink_id: CellId, target_id: CellId) {
        let sink = self.cells.get(&sink_id).unwrap().borrow();
        let mut target = self.cells.get(&target_id).unwrap().borrow_mut();
        (*target).set_id = (*sink).set_id;
    }

    fn connected(&self, id: CellId, other_id: CellId) -> bool {
        let cell = self.cells.get(&id).unwrap().borrow();
        let other = self.cells.get(&other_id).unwrap().borrow();
        (*cell).set_id == (*other).set_id
    }

    fn get_cell_pos(&self, id: CellId) -> Pos {
        let cell = self.cells.get(&id).unwrap().borrow();
        (*cell).pos
    }

    fn sets(&self) -> HashMap<SetId, Vec<CellId>> {
        let mut sets: HashMap<SetId, Vec<CellId>> = HashMap::new();

        self.cells.iter().for_each(|(id, cell)| {
            let cell = cell.borrow();

            if let Some(cells) = sets.get_mut(&cell.set_id) {
                (*cells).push(*id);
            } else {
                sets.insert(cell.set_id, vec![*id]);
            }
        });

        sets
    }
}

pub struct Eller {
    generator: Generator,
}

impl Eller {
    /// Randomly joins adjacent cells, but only if they are not in the same set
    fn connect_disjoint_sets(&mut self, state: &mut State, is_last_row: bool) {
        let mut rng = rand::thread_rng();

        for c in 1..state.width {
            let cell_id = CellId(c);
            let next_cell_id = CellId(c + 1);

            if state.connected(cell_id, next_cell_id) || (!is_last_row && rng.gen::<bool>()) {
                continue;
            }

            state.connect(cell_id, next_cell_id);
            let (x, y) = state.get_cell_pos(cell_id);
            self.generator.grid.carve_passage((x, y), Pole::E).unwrap();

            self.generator.highlights.push((x, y));
            self.generator.make_snapshot();
        }
    }

    /// For each set, creates at least one vertical connection downward to the next row
    fn add_vertical_connections(&mut self, state: &mut State, is_last_row: bool) -> State {
        let mut next_state = state.next();

        if is_last_row {
            return next_state.populate();
        }

        for (set_id, cells) in state.sets() {
            for cell_id in self.cells_to_connect(cells) {
                let (x, y) = state.get_cell_pos(cell_id);
                self.generator.grid.carve_passage((x, y), Pole::S).unwrap();
                next_state.add(cell_id, set_id, (x, y + 1));

                self.generator.highlights.push((x, y));
                self.generator.highlights.push((x, y + 1));
                self.generator.make_snapshot();
            }
        }

        next_state.populate()
    }

    /// Selects random cells to carve vertical passages from
    fn cells_to_connect(&self, cells: Vec<CellId>) -> Vec<CellId> {
        let mut rng = rand::thread_rng();

        let mut cells = cells.clone();
        cells.shuffle(&mut rng);

        let connect_count = if cells.len() >= 2 {
            rng.gen_range(1..cells.len())
        } else {
            1
        };

        cells
            .iter()
            .take(connect_count)
            .cloned()
            .collect::<Vec<CellId>>()
    }
}

impl IGenerator for Eller {
    fn init(width: usize, height: usize) -> Self {
        let generator = Generator::new(width, height);
        Self { generator }
    }

    fn run(&mut self) -> Vec<MazeSnapshot> {
        let width = self.generator.grid.width();
        let height = self.generator.grid.height();

        let mut state = State::new(0, None, width).populate();

        for row in 0..height {
            let is_last_row = row == height - 1;
            self.connect_disjoint_sets(&mut state, is_last_row);
            state = self.add_vertical_connections(&mut state, is_last_row);

            self.generator.highlights.clear();
            self.generator.make_snapshot();
        }

        self.generator.get_snapshots()
    }
}
