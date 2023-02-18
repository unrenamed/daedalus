pub mod cell;

use super::utils::types::Pos;
use cell::Cell;
use cell::CellStatus;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TransitError {
    pub pos: Pos,
    pub reason: String,
}

impl fmt::Display for TransitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y) = self.pos;

        write!(
            f,
            "Cannot move to a cell. Reason: {}. Pos: x = {}, y = {}",
            self.reason, x, y
        )
    }
}

type Cells = Vec<Vec<Cell>>;
type CellStatuses = Vec<Vec<CellStatus>>;
type TransitResult<T> = Result<T, TransitError>;

pub struct Grid {
    width: usize,
    height: usize,
    cells: Cells,
    cell_statuses: CellStatuses,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            cells: vec![vec![Cell::default(); width]; height],
            cell_statuses: vec![vec![CellStatus::default(); width]; height],
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn mark_cell(&mut self, pos: Pos) {
        let (x, y) = pos;
        self.cell_statuses[y][x].mark()
    }

    pub fn visit_cell(&mut self, pos: Pos) {
        let (x, y) = pos;
        self.cell_statuses[y][x].visit()
    }

    pub fn is_cell_visited(&self, pos: Pos) -> bool {
        let (x, y) = pos;
        self.cell_statuses[y][x].visited()
    }

    pub fn is_cell_marked(&self, pos: Pos) -> bool {
        let (x, y) = pos;
        self.cell_statuses[y][x].marked()
    }

    pub fn is_cell_carved(&self, pos: Pos, direction: Cell) -> bool {
        let (x, y) = pos;
        self.cells[y][x].contains(direction)
    }

    pub fn carve_passage(&mut self, pos: Pos, direction: Cell) -> TransitResult<Pos> {
        let (x, y) = pos;
        let (nx, ny) = self.get_next_cell_pos(pos, direction)?;

        match direction {
            Cell::NORTH => {
                self.cells[y][x] |= Cell::NORTH;
                self.cells[ny][nx] |= Cell::SOUTH;
            }
            Cell::SOUTH => {
                self.cells[y][x] |= Cell::SOUTH;
                self.cells[ny][nx] |= Cell::NORTH;
            }
            Cell::EAST => {
                self.cells[y][x] |= Cell::EAST;
                self.cells[ny][nx] |= Cell::WEST;
            }
            Cell::WEST => {
                self.cells[y][x] |= Cell::WEST;
                self.cells[ny][nx] |= Cell::EAST;
            }
            _ => (),
        }

        self.visit_cell(pos);
        self.visit_cell((nx, ny));

        Ok((nx, ny))
    }

    pub fn get_next_cell_pos(&mut self, pos: Pos, direction: Cell) -> TransitResult<Pos> {
        self.validate_transit(pos, direction)?;

        let (x, y) = pos;
        let (nx, ny) = match direction {
            Cell::NORTH => (x, y - 1),
            Cell::SOUTH => (x, y + 1),
            Cell::WEST => (x - 1, y),
            Cell::EAST => (x + 1, y),
            _ => (x, y),
        };
        Ok((nx, ny))
    }

    fn validate_transit(&self, pos: Pos, direction: Cell) -> TransitResult<()> {
        let (x, y) = pos;
        let reason = match direction {
            Cell::NORTH if y < 1 => Some("First row in the grid cannot go North"),
            Cell::SOUTH if y + 1 == self.height => Some("Last row in the grid cannot go South"),
            Cell::WEST if x < 1 => Some("First cell in a row cannot go West"),
            Cell::EAST if x + 1 == self.width => Some("Last column in the grid cannot go East"),
            _ => None,
        };

        if reason.is_none() {
            return Ok(());
        }

        return Err(TransitError {
            pos,
            reason: reason.unwrap().to_string(),
        });
    }
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Grid {
            width: self.width,
            height: self.height,
            cells: self.cells.to_vec(),
            cell_statuses: self.cell_statuses.to_vec(),
        }
    }
}
