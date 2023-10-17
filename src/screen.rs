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

pub struct Screen {
    pub size: Size<u16>,
    pub cursor: Point<u16>,
    pub cursor_style: CursorStyle,
    pub rows: Vec<Vec<Cell>>,
}

impl Screen {
    pub fn new(rows: u16, columns: u16) -> Self {
        let cursor = Point { x: 0, y: 0 };

        let mut screen = Self {
            cursor,
            cursor_style: CursorStyle::BlinkingBlock,
            size: Size {
                width: columns,
                height: rows,
            },
            rows: vec![],
        };

        for row_index in 0..screen.size.height {
            let mut row: Vec<Cell> = vec![];

            for _ in 0..screen.size.width {
                let mut cell = Cell::new();

                if row_index == 0 {
                    cell.background_color = (125, 125, 125);
                }

                row.push(cell);
            }

            screen.rows.push(row);
        }

        screen
    }

    pub fn from(editor: &Editor) -> Self {
        let mut screen = Screen::new(editor.area.height, editor.area.width);

        let tab = editor.get_active_tab();
        let buffer = tab.get_active_buffer();

        screen.cursor = buffer.get_cursor_screen_pos();

        match buffer.mode {
            BufferMode::Normal => screen.cursor_style = CursorStyle::BlinkingBlock,
            BufferMode::Visual => screen.cursor_style = CursorStyle::BlinkingBlock,
            BufferMode::Insert => screen.cursor_style = CursorStyle::BlinkingBar,
            BufferMode::Command => screen.cursor_style = CursorStyle::BlinkingBar,
        }

        match &buffer.file_name {
            Some(name) => screen.print(0, 0, &name),
            None => screen.print(0, 0, "[No Name]"),
        }

        let info_area_width = buffer.info_area.width as usize - 2;

        for y in 0..buffer.area.height {
            let row_index = buffer.scroll.y + y as usize;
            match buffer.get_line_visible_text(row_index) {
                Some(text) => {
                    screen.print(
                        y + 1,
                        0,
                        &format!(" {:>2$} {}", row_index + 1, text, info_area_width),
                    );
                }
                None => screen.print(y + 1, 0, &format!("~")),
            }
        }

        if let BufferMode::Command = buffer.mode {
            let command_row = screen.size.height - 1;
            screen.print(command_row, 0, &format!(":{}", editor.command.text));

            screen.cursor.x = editor.command.cursor_x as u16 + 1;
            screen.cursor.y = command_row;
        } else {
            screen.print(screen.size.height - 1, 0, &format!("{}", buffer.mode));
        }

        screen
    }
}

impl Screen {
    pub fn print(&mut self, row: u16, column: u16, text: &str) {
        let columns = self.rows.get_mut(row as usize).unwrap();
        let mut column_index = column as usize;
        let mut chars = text.chars();

        while let Some(c) = chars.next() {
            columns.get_mut(column_index).unwrap().char = c;
            column_index += 1;
        }
    }
}
