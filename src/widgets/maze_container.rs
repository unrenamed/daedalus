use tui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Context, Line, Points, Rectangle},
        Block, BorderType, Borders, Row, Table, Widget,
    },
};

use crate::{
    grid::{cell::Cell, pole::Pole, Grid},
    utils::types::Coords,
};

pub struct MazeContainer<'a> {
    pub block: Option<Block<'a>>,
    pub grid: &'a Grid,
    pub stack: &'a Vec<Coords>,
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

        if widget_area.height < 1 {
            return;
        }

        self.display_grid(widget_area, buf);
    }
}

impl<'a> MazeContainer<'a> {
    pub fn new(grid: &'a Grid, stack: &'a Vec<Coords>) -> Self {
        MazeContainer {
            block: None,
            stack,
            grid,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> MazeContainer<'a> {
        self.block = Some(block);
        self
    }

    pub fn display_grid(&mut self, area: Rect, buf: &mut Buffer) {
        let x_margin = 3u16;
        let y_margin = 5u16;
        let grid_width = self.grid.width() as u16;
        let grid_height = self.grid.height() as u16;

        buf.get_mut(x_margin, y_margin)
            .set_fg(Color::Green)
            .set_symbol(" ");
        for x in 1..=grid_width * 2 - 1 {
            buf.get_mut(x_margin + x as u16, y_margin)
                .set_fg(Color::Green)
                .set_symbol("_");
        }

        for (y, row) in self.grid.cells().iter().enumerate() {
            buf.get_mut(x_margin, y_margin + y as u16 + 1)
                .set_fg(Color::Green)
                .set_symbol("|");

            for (x, cell) in row.iter().enumerate() {
                let walls = self.grid.get_cell((x, y)).get_walls();

                // draw southern wall
                if walls.carved(Pole::S) {
                    if self.stack.contains(&(x, y)) {
                        buf.get_mut(x_margin + x as u16 * 2 + 1, y_margin + y as u16 + 1)
                            .set_bg(Color::Red)
                            .set_symbol(" ");
                    } else {
                        buf.get_mut(x_margin + x as u16 * 2 + 1, y_margin + y as u16 + 1)
                            .set_bg(Color::Reset)
                            .set_symbol(" ");
                    }
                } else {
                    if self.stack.contains(&(x, y)) {
                        buf.get_mut(x_margin + x as u16 * 2 + 1, y_margin + y as u16 + 1)
                            .set_fg(Color::Green)
                            .set_bg(Color::Red)
                            .set_symbol("_");
                    } else {
                        buf.get_mut(x_margin + x as u16 * 2 + 1, y_margin + y as u16 + 1)
                            .set_fg(Color::Green)
                            .set_bg(Color::Reset)
                            .set_symbol("_");
                    }
                }

                // draw eastern wall
                if walls.carved(Pole::E) {
                    let next_cell_walls = self.grid.get_cell((x + 1, y)).get_walls();
                    if walls.carved(Pole::S) || next_cell_walls.carved(Pole::S) {
                        if self.stack.contains(&(x, y)) {
                            buf.get_mut(x_margin + x as u16 * 2 + 2, y_margin + y as u16 + 1)
                                .set_bg(Color::Red)
                                .set_symbol(" ");
                        } else {
                            buf.get_mut(x_margin + x as u16 * 2 + 2, y_margin + y as u16 + 1)
                                .set_bg(Color::Reset)
                                .set_symbol(" ");
                        }
                    } else {
                        if self.stack.contains(&(x, y)) {
                            buf.get_mut(x_margin + x as u16 * 2 + 2, y_margin + y as u16 + 1)
                                .set_fg(Color::Green)
                                .set_bg(Color::Red)
                                .set_symbol("_");
                        } else {
                            buf.get_mut(x_margin + x as u16 * 2 + 2, y_margin + y as u16 + 1)
                                .set_fg(Color::Green)
                                .set_bg(Color::Reset)
                                .set_symbol("_");
                        }
                    }
                } else {
                    buf.get_mut(x_margin + x as u16 * 2 + 2, y_margin + y as u16 + 1)
                        .set_fg(Color::Green)
                        .set_symbol("|");
                }
            }
        }
    }
}
