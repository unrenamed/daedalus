use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{Block, Widget},
};

use crate::app::{
    grid::{Grid, cell::Cell},
    utils::types::Pos,
};

pub struct MazeContainer<'a> {
    pub block: Option<Block<'a>>,
    pub grid: &'a Grid,
    pub highlights: &'a Vec<Pos>,
}

impl<'a> Widget for MazeContainer<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let widget_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        // plus 1 to take into account the western maze wall
        let min_maze_height = self.grid.height() as u16 + 1;
        // plus 1 to take into account the western maze wall; multilple by 2 since one cell takes at least 2 characters
        let min_maze_width = (self.grid.width() as u16 + 1) * 2;
        // don't render the grid if widget area size is too small to display the grid of factor 1
        if widget_area.height < min_maze_height || widget_area.width < min_maze_width {
            return;
        }

        self.display_grid(widget_area, buf);
    }
}

impl<'a> MazeContainer<'a> {
    pub fn new(grid: &'a Grid, highlights: &'a Vec<Pos>) -> Self {
        MazeContainer {
            block: None,
            highlights,
            grid,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> MazeContainer<'a> {
        self.block = Some(block);
        self
    }

    pub fn display_grid(&mut self, area: Rect, buf: &mut Buffer) {
        let grow_factor = self.get_grid_grow_factor(area, self.grid);
        let (x_margin, y_margin) = self.get_grid_margins(grow_factor, area, self.grid);
        let grid_width = self.grid.width() as u16;
        let grid_height = self.grid.height() as u16;
        let maze_width = grid_width * grow_factor * 2;
        let maze_height = grid_height * grow_factor;

        self.add_passage((x_margin, y_margin), None, buf);

        // draw top border
        for x in 0..maze_width - 1 {
            self.add_horizontal_wall((x_margin + x as u16 + 1, y_margin), None, buf);
        }

        for y in 0..maze_height {
            self.add_vertical_wall((x_margin, y_margin + y as u16 + 1), buf);

            for x in 0..maze_width {
                // X coordinate including maring on the axis X
                let nx = x_margin + x as u16 + 1;
                // Y coordinate including maring on the axis Y
                let ny = y_margin + y as u16 + 1;
                // X coordinate of a cell in the grid
                let cx = (x as f64 / grow_factor as f64 / 2.0).floor() as usize;
                // Y coordinate of a cell in the grid
                let cy = (y as f64 / grow_factor as f64).floor() as usize;
                // Indicates if a row is a last row of a grid cell
                let is_last_row = (y as f64 + 1.0) / grow_factor as f64 == cy as f64 + 1.0;
                // Indicates if a column is a last column of a grid cell
                let is_last_col = (x as f64 + 1.0) / grow_factor as f64 / 2.0 == cx as f64 + 1.0;

                match (is_last_row, is_last_col) {
                    (false, false) => self.add_passage((nx, ny), Some((cx, cy)), buf),
                    (false, true) => {
                        if self.grid.is_cell_carved((cx, cy), Cell::EAST) {
                            self.add_passage((nx, ny), Some((cx, cy)), buf);
                        } else {
                            self.add_vertical_wall((nx, ny), buf);
                        }
                    }
                    (true, false) => {
                        if self.grid.is_cell_carved((cx, cy), Cell::SOUTH) {
                            self.add_passage((nx, ny), Some((cx, cy)), buf);
                        } else {
                            self.add_horizontal_wall((nx, ny), Some((cx, cy)), buf);
                        }
                    }
                    (true, true) => {
                        if self.grid.is_cell_carved((cx, cy), Cell::EAST) {
                            if self.grid.is_cell_carved((cx, cy), Cell::SOUTH)
                                || self.next_cell_carved_south(cx, cy, self.grid)
                            {
                                self.add_passage((nx, ny), Some((cx, cy)), buf);
                            } else {
                                self.add_horizontal_wall((nx, ny), Some((cx, cy)), buf);
                            }
                        } else {
                            self.add_vertical_wall((nx, ny), buf);
                        }
                    }
                }
            }
        }
    }

    fn get_grid_grow_factor(&self, area: Rect, grid: &Grid) -> u16 {
        let margin = 2.0;

        let grid_width = grid.width() as f64 * 2.0;
        let grid_height = grid.height() as f64;
        let width_factor = (area.width as f64 / (grid_width + margin)).floor().max(1.0);
        let height_factor = (area.height as f64 / (grid_height + margin)).floor().max(1.0);

        width_factor.min(height_factor) as u16
    }

    fn get_grid_margins(&self, grow_factor: u16, area: Rect, grid: &Grid) -> (u16, u16) {
        let grid_width = grid.width() as u16 * 2u16 * grow_factor;
        let grid_height = (grid.height() as u16 + 1) * grow_factor;

        let x_margin = (area.width - grid_width) / 2;
        let y_margin = (area.height - grid_height) / 2;

        (x_margin + area.left(), y_margin + area.top())
    }

    fn add_vertical_wall(&self, (x, y): (u16, u16), buf: &mut Buffer) {
        buf.get_mut(x, y).set_fg(Color::Green).set_symbol("|");
    }

    fn add_horizontal_wall(&self, (x, y): (u16, u16), cell_pos: Option<Pos>, buf: &mut Buffer) {
        let cell = buf.get_mut(x, y);
        cell.set_fg(Color::Green).set_symbol("_");

        if let Some((cx, cy)) = cell_pos {
            if self.highlights.contains(&(cx, cy)) {
                cell.set_bg(Color::Red);
            }
        }
    }

    fn add_passage(&self, (x, y): (u16, u16), cell_pos: Option<Pos>, buf: &mut Buffer) {
        let cell = buf.get_mut(x, y);
        cell.set_fg(Color::Green).set_symbol(" ");

        if let Some((cx, cy)) = cell_pos {
            if self.highlights.contains(&(cx, cy)) {
                cell.set_bg(Color::Red);
            }
        }
    }

    fn next_cell_carved_south(&self, cx: usize, cy: usize, grid: &Grid) -> bool {
        if cx + 1 >= grid.height() {
            return false;
        }
        grid.is_cell_carved((cx + 1, cy), Cell::SOUTH)
    }
}
