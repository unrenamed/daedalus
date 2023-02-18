pub mod cell;

use super::utils::types::Pos;
use cell::{Cell, CellStatus};
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

type TransitResult<T> = Result<T, TransitError>;

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    cell_statuses: Vec<CellStatus>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            cells: vec![Cell::default(); width * height],
            cell_statuses: vec![CellStatus::default(); width * height],
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn mark_cell(&mut self, pos: Pos) {
        self.get_cell_status_mut(pos).mark()
    }

    pub fn visit_cell(&mut self, pos: Pos) {
        self.get_cell_status_mut(pos).visit()
    }

    pub fn is_cell_visited(&self, pos: Pos) -> bool {
        self.get_cell_status(pos).visited()
    }

    pub fn is_cell_marked(&self, pos: Pos) -> bool {
        self.get_cell_status(pos).marked()
    }

    pub fn is_cell_carved(&self, pos: Pos, direction: Cell) -> bool {
        self.get_cell(pos).contains(direction)
    }

    pub fn carve_passage(&mut self, pos: Pos, direction: Cell) -> TransitResult<Pos> {
        let npos = self.get_next_cell_pos(pos, direction)?;

        match direction {
            Cell::NORTH => {
                *self.get_cell_mut(pos) |= Cell::NORTH;
                *self.get_cell_mut(npos) |= Cell::SOUTH;
            }
            Cell::SOUTH => {
                *self.get_cell_mut(pos) |= Cell::SOUTH;
                *self.get_cell_mut(npos) |= Cell::NORTH;
            }
            Cell::EAST => {
                *self.get_cell_mut(pos) |= Cell::EAST;
                *self.get_cell_mut(npos) |= Cell::WEST;
            }
            Cell::WEST => {
                *self.get_cell_mut(pos) |= Cell::WEST;
                *self.get_cell_mut(npos) |= Cell::EAST;
            }
            _ => (),
        }

        self.visit_cell(pos);
        self.visit_cell(npos);

        Ok(npos)
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

    fn get_cell_status(&self, pos: Pos) -> &CellStatus {
        let (x, y) = pos;
        &self.cell_statuses[y * self.width + x]
    }

    fn get_cell_status_mut(&mut self, pos: Pos) -> &mut CellStatus {
        let (x, y) = pos;
        &mut self.cell_statuses[y * self.width + x]
    }

    fn get_cell(&self, pos: Pos) -> &Cell {
        let (x, y) = pos;
        &self.cells[y * self.width + x]
    }

    fn get_cell_mut(&mut self, pos: Pos) -> &mut Cell {
        let (x, y) = pos;
        &mut self.cells[y * self.width + x]
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
