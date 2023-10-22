use crate::{
    core::{Color, Style},
    screen::Screen,
};

#[derive(Clone, Eq, PartialEq)]
pub struct Cell {
    pub char: char,
    pub style: Style,
}

impl Cell {
    pub fn new(fg: Color, bg: Color) -> Self {
        Self {
            char: ' ',
            style: Style {
                fg,
                bg,
                bold: false,
                italic: false,
                underline: false,
            },
        }
    }
}

impl Screen {
    pub fn get_cell(&self, row: u16, column: u16) -> Option<&Cell> {
        if let Some(cells) = self.rows.get(row as usize) {
            if let Some(cell) = cells.get(column as usize) {
                return Some(cell);
            }
        }

        None
    }

    pub fn get_cell_mut(&mut self, row: u16, column: u16) -> Option<&mut Cell> {
        if let Some(cells) = self.rows.get_mut(row as usize) {
            if let Some(cell) = cells.get_mut(column as usize) {
                return Some(cell);
            }
        }

        None
    }
}
