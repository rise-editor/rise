use crate::{core::style::Style, screen::Screen};

impl Screen {
    pub fn print_text(&mut self, row: u16, column: u16, text: &str, style: Style) {
        let columns = self.rows.get_mut(row as usize).unwrap();
        let mut column_index = column as usize;
        let mut chars = text.chars();

        while let Some(c) = chars.next() {
            let cell = columns.get_mut(column_index).unwrap();
            cell.char = c;
            cell.style = style.clone();
            column_index += 1;
        }
    }
}
