use std::collections::HashMap;

use crate::{core::style::Style, screen::Screen, theme::THEME_ONE as T};

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

    pub fn print_buffer_text(
        &mut self,
        replace_chars: &HashMap<char, char>,
        row: u16,
        column: u16,
        text: &str,
    ) {
        let columns = self.rows.get_mut(row as usize).unwrap();
        let mut column_index = column as usize;
        for c in text.chars() {
            let cell = columns.get_mut(column_index).unwrap();
            cell.char = if let Some(replace_char) = replace_chars.get(&c) {
                replace_char.clone()
            } else {
                c
            };
            cell.style = if c == ' ' {
                Style::new(T.text_whitespace_fg, T.text_whitespace_bg)
            } else {
                Style::new(T.text_fg, T.text_bg)
            };
            column_index += 1;
        }
    }
}
