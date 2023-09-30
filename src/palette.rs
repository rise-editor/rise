use crate::{
    buffer::mode::BufferMode,
    core::{Point, Size},
    editor::Editor,
    terminal::CursorStyle,
};

pub struct Cell {
    pub char: char,
    pub color: (u8, u8, u8),
    pub background_color: (u8, u8, u8),
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
}

pub struct Palette {
    pub size: Size<u16>,
    pub cursor: Point<u16>,
    pub cursor_style: CursorStyle,
    pub rows: Vec<Vec<Cell>>,
}

impl Palette {
    pub fn from(rows: u16, columns: u16) -> Self {
        let cursor = Point { x: 0, y: 0 };

        let mut palette = Palette {
            cursor,
            cursor_style: CursorStyle::BlinkingBlock,
            size: Size {
                width: columns,
                height: rows,
            },
            rows: vec![],
        };

        for _ in 0..palette.size.height {
            let mut row: Vec<Cell> = vec![];

            for _ in 0..palette.size.width {
                let cell = Cell {
                    char: ' ',
                    color: (255, 255, 255),
                    background_color: (0, 0, 0),
                    bold: false,
                    italic: false,
                    underline: false,
                };

                row.push(cell);
            }

            palette.rows.push(row);
        }

        palette
    }

    pub fn new(editor: &Editor) -> Self {
        let window = editor.get_active_window();
        let mut palette = Palette::from(window.size.height, window.size.width);

        palette.cursor.x = window.get_active_buffer_visible_x(window.get_active_buffer().cursor.x);
        palette.cursor.y = window.get_active_buffer_visible_y(window.get_active_buffer().cursor.y);

        let buffer = window.get_active_buffer();

        match buffer.mode {
            BufferMode::Insert => palette.cursor_style = CursorStyle::BlinkingBar,
            BufferMode::Command => palette.cursor_style = CursorStyle::BlinkingBar,
            _ => {}
        }

        for y in 0..buffer.area.height {
            let line = buffer.get_line_visible_text(buffer.scroll.y + y as usize);

            palette.print(y + 1, 0, &line);
        }

        palette.print(palette.size.height - 1, 0, &format!("{}", buffer.mode));

        if let BufferMode::Command = buffer.mode {
            palette.print(
                palette.size.height - 2,
                0,
                &format!(":{}", buffer.command.text),
            );
        }

        palette
    }
}

impl Palette {
    fn print(&mut self, row: u16, column: u16, text: &String) {
        let columns = self.rows.get_mut(row as usize).unwrap();
        let mut column_index = column as usize;
        let mut chars = text.chars();

        while let Some(c) = chars.next() {
            columns.get_mut(column_index).unwrap().char = c;
            column_index += 1;
        }
    }
}
