use crate::{
    buffer::mode::BufferMode,
    core::{Point, Size},
    editor::Editor,
    terminal::CursorStyle,
};

#[derive(Clone, Eq, PartialEq)]
pub struct Cell {
    pub char: char,
    pub color: (u8, u8, u8),
    pub background_color: (u8, u8, u8),
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            char: ' ',
            color: (255, 255, 255),
            background_color: (34, 39, 46),
            bold: false,
            underline: false,
            italic: false,
        }
    }
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

        for row_index in 0..palette.size.height {
            let mut row: Vec<Cell> = vec![];

            for _ in 0..palette.size.width {
                let mut cell = Cell::new();

                if row_index == 0 {
                    cell.background_color = (125, 125, 125);
                }

                row.push(cell);
            }

            palette.rows.push(row);
        }

        palette
    }

    pub fn new(editor: &Editor) -> Self {
        let window = editor.get_active_window();
        let mut palette = Palette::from(editor.size.height, editor.size.width);

        palette.cursor.x =
            window.get_active_buffer_visible_x(window.get_active_buffer().cursor.x) + 5;
        palette.cursor.y = window.get_active_buffer_visible_y(window.get_active_buffer().cursor.y);

        let buffer = window.get_active_buffer();

        match buffer.mode {
            BufferMode::Insert => palette.cursor_style = CursorStyle::BlinkingBar,
            BufferMode::Command => palette.cursor_style = CursorStyle::BlinkingBar,
            _ => {}
        }

        match &buffer.file_name {
            Some(name) => palette.print(0, 0, &format!("{}", name)),
            None => palette.print(0, 0, &String::from("[No Name]")),
        }

        let number_column_width = buffer.lines.len().to_string().len();

        for y in 0..buffer.area.height {
            let row_index = buffer.scroll.y + y as usize;
            match buffer.get_line_visible_text(row_index) {
                Some(text) => {
                    palette.print(
                        y + 1,
                        0,
                        &format!(" {:>2$} {}", row_index + 1, text, number_column_width),
                    );
                }
                None => palette.print(y + 1, 0, &format!("~")),
            }
        }

        palette.print(palette.size.height - 1, 0, &format!("{}", buffer.mode));

        if let BufferMode::Command = buffer.mode {
            let command_row = palette.size.height - 1;
            palette.clear_row(command_row);
            palette.print(command_row, 0, &format!(":{}", buffer.command.text));

            palette.cursor.x = buffer.command.cursor_x as u16 + 1;
            palette.cursor.y = command_row;
        }

        palette
    }
}

impl Palette {
    pub fn clear_row(&mut self, row: u16) {
        let columns = self.rows.get_mut(row as usize).unwrap();

        for column in columns.iter_mut() {
            column.char = ' ';
        }
    }

    pub fn print(&mut self, row: u16, column: u16, text: &String) {
        let columns = self.rows.get_mut(row as usize).unwrap();
        let mut column_index = column as usize;
        let mut chars = text.chars();

        while let Some(c) = chars.next() {
            columns.get_mut(column_index).unwrap().char = c;
            column_index += 1;
        }
    }
}
