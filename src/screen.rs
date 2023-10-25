pub mod cell;
pub mod print_buffer;
pub mod print_statusbar;
pub mod print_tabs;
pub mod print_text;

use crate::{
    buffer::mode::BufferMode,
    core::{point::Point, rectangle::Rectangle, size::Size, style::Style},
    editor::Editor,
    screen::cell::Cell,
    terminal::CursorStyle,
    theme::{THEME_ONE as T, WHITE},
};

pub struct Screen {
    pub size: Size<u16>,
    pub cursor: Point<u16>,
    pub cursor_style: CursorStyle,
    pub rows: Vec<Vec<Cell>>,
}

impl Screen {
    pub fn new(size: Size<u16>) -> Self {
        let mut screen = Self {
            cursor: Point { x: 0, y: 0 },
            cursor_style: CursorStyle::BlinkingBlock,
            size,
            rows: vec![],
        };

        for _ in 0..screen.size.height {
            let mut row: Vec<Cell> = vec![];

            for _ in 0..screen.size.width {
                row.push(Cell::new(WHITE, T.bg));
            }

            screen.rows.push(row);
        }

        screen
    }

    pub fn from(editor: &Editor) -> Self {
        let mut screen = Screen::new(editor.area.to_size());

        screen.cursor_style = match editor.get_active_buffer_or_popup().mode {
            BufferMode::Normal => CursorStyle::BlinkingBlock,
            BufferMode::Visual => CursorStyle::BlinkingBlock,
            BufferMode::Insert => CursorStyle::BlinkingBar,
            BufferMode::Command => CursorStyle::BlinkingBar,
            BufferMode::Find => CursorStyle::BlinkingBar,
        };

        screen.cursor = match editor.get_active_buffer_or_popup().mode {
            BufferMode::Command | BufferMode::Find => Point {
                y: editor.status_area.y,
                x: (editor.status_area.x + 1 + editor.input.cursor_x as u16),
            },
            _ => editor.get_active_buffer_or_popup().get_cursor_screen_pos(),
        };

        screen.print_tabs(editor);

        let tab = editor.get_active_tab();
        let buffer = tab.get_active_buffer();

        screen.print_buffer(&buffer);

        for popup in buffer.popups.iter() {
            screen.print_buffer(popup);
        }

        screen.print_statusbar(editor);

        screen
    }
}

impl Screen {
    pub fn set_style(&mut self, row: u16, from: u16, to: u16, style: Style) {
        let row = self.rows.get_mut(row as usize).unwrap();
        for column in from..(to + 1) {
            let cell = row.get_mut(column as usize).unwrap();
            cell.style = style.clone();
        }
    }

    pub fn clear_area(&mut self, area: Rectangle<u16>) {
        for y in 0..area.height {
            for x in 0..area.width {
                let row = area.y + y;
                let column = area.x + x;
                let cell = self.get_cell_mut(row, column).unwrap();

                cell.char = ' ';
                cell.style.bg = T.bg;
            }
        }
    }
}
