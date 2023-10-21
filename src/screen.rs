use crate::{
    buffer::{mode::BufferMode, Buffer},
    core::{Point, Rectangle, Size},
    editor::Editor,
    terminal::CursorStyle,
    theme::{Color, THEME_ONE as T, WHITE},
};

#[derive(Clone, Eq, PartialEq)]
pub struct Cell {
    pub char: char,
    pub color: Color,
    pub background_color: Color,
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
}

impl Cell {
    pub fn new(fg: Color, bg: Color) -> Self {
        Self {
            char: ' ',
            color: fg,
            background_color: bg,
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
        let mut screen = Self {
            cursor: Point { x: 0, y: 0 },
            cursor_style: CursorStyle::BlinkingBlock,
            size: Size {
                width: columns,
                height: rows,
            },
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
        let mut screen = Screen::new(editor.area.height, editor.area.width);

        let width = screen.size.width;
        let height = screen.size.height;

        screen.cursor = editor.get_active_buffer_or_popup().get_cursor_screen_pos();

        match editor.get_active_buffer_or_popup().mode {
            BufferMode::Normal => screen.cursor_style = CursorStyle::BlinkingBlock,
            BufferMode::Visual => screen.cursor_style = CursorStyle::BlinkingBlock,
            BufferMode::Insert => screen.cursor_style = CursorStyle::BlinkingBar,
            BufferMode::Command => screen.cursor_style = CursorStyle::BlinkingBar,
        }

        let tab = editor.get_active_tab();
        let buffer = tab.get_active_buffer();

        screen.paint_range(
            Point { y: 0, x: 0 },
            Point { y: 0, x: width - 1 },
            WHITE,
            T.tab_line_bg,
        );
        screen.paint_range(
            Point {
                y: height - 1,
                x: 0,
            },
            Point {
                y: height - 1,
                x: width - 1,
            },
            WHITE,
            T.status_line_bg,
        );

        match &buffer.file_name {
            Some(name) => screen.print(0, 0, &name, T.tab_selected_fg, T.tab_selected_bg),
            None => screen.print(0, 0, "[No Name]", T.tab_selected_fg, T.tab_selected_bg),
        }

        screen.print_buffer(&buffer);

        for popup in buffer.popups.iter() {
            screen.print_buffer(popup);
        }

        match buffer.mode {
            BufferMode::Normal => {
                screen.print(
                    screen.size.height - 1,
                    0,
                    &format!("{}", buffer.mode),
                    T.status_normal_mode_fg,
                    T.status_normal_mode_bg,
                );
            }
            BufferMode::Insert => {
                screen.print(
                    screen.size.height - 1,
                    0,
                    &format!("{}", buffer.mode),
                    T.status_insert_mode_fg,
                    T.status_insert_mode_bg,
                );
            }
            BufferMode::Visual => {
                screen.print(
                    screen.size.height - 1,
                    0,
                    &format!("{}", buffer.mode),
                    T.status_visual_mode_fg,
                    T.status_visual_mode_bg,
                );
                // TODO: Move calculation and improve
                let pos_start = Point {
                    x: buffer.text_area.x + (buffer.select.start.x - buffer.scroll.x) as u16,
                    y: buffer.text_area.y + (buffer.select.start.y - buffer.scroll.y) as u16,
                };
                let pos_end = Point {
                    x: buffer.text_area.x + (buffer.cursor.x - buffer.scroll.x) as u16,
                    y: buffer.text_area.y + (buffer.cursor.y - buffer.scroll.y) as u16,
                };
                screen.paint_range(pos_start, pos_end, T.text_selected_fg, T.text_selected_bg);
            }
            _ => {}
        }

        if let BufferMode::Command = buffer.mode {
            let command_row = screen.size.height - 1;
            screen.print(
                command_row,
                0,
                &format!(":{}", editor.command.text),
                WHITE,
                T.status_line_bg,
            );

            screen.cursor.x = editor.command.cursor_x as u16 + 1;
            screen.cursor.y = command_row;
        }

        screen
    }

    pub fn cell(&self, row: u16, column: u16) -> Option<&Cell> {
        if let Some(cells) = self.rows.get(row as usize) {
            if let Some(cell) = cells.get(column as usize) {
                return Some(cell);
            }
        }

        None
    }

    pub fn print_buffer(&mut self, buffer: &Buffer) {
        self.clear_square(buffer.area.clone());

        for y in 0..buffer.area.height {
            let row_index = buffer.scroll.y + y as usize;
            match buffer.get_line_visible_text(row_index) {
                Some(text) => {
                    self.print(
                        buffer.area.y + y,
                        buffer.area.x,
                        &format!(
                            " {:>1$} ",
                            row_index + 1,
                            buffer.info_area.width as usize - 2
                        ),
                        T.info_column_fg,
                        T.info_column_bg,
                    );
                    self.print(
                        buffer.area.y + y,
                        buffer.area.x + buffer.info_area.width,
                        &text,
                        T.text_fg,
                        T.text_bg,
                    );
                }
                None => self.print(
                    buffer.area.y + y,
                    buffer.area.x,
                    "~",
                    T.info_column_fg,
                    T.bg,
                ),
            }
        }
    }
}

impl Screen {
    pub fn print(&mut self, row: u16, column: u16, text: &str, fg: Color, bg: Color) {
        let columns = self.rows.get_mut(row as usize).unwrap();
        let mut column_index = column as usize;
        let mut chars = text.chars();

        while let Some(c) = chars.next() {
            let cell = columns.get_mut(column_index).unwrap();
            cell.char = c;
            cell.color = fg;
            cell.background_color = bg;
            column_index += 1;
        }
    }

    pub fn paint_range(
        &mut self,
        position_start: Point<u16>,
        position_end: Point<u16>,
        fg: Color,
        bg: Color,
    ) {
        for row in position_start.y..(position_end.y + 1) {
            let column_start = if row == position_start.y {
                position_start.x
            } else {
                0
            };

            let column_end = if row == position_end.y {
                position_end.x
            } else {
                self.size.width - 1
            };

            let columns = self.rows.get_mut(row as usize).unwrap();

            for column in column_start..(column_end + 1) {
                let cell = columns.get_mut(column as usize).unwrap();
                cell.color = fg;
                cell.background_color = bg;
            }
        }
    }

    pub fn clear_square(&mut self, area: Rectangle<u16>) {
        for row in 0..area.height {
            for column in 0..area.width {
                let cell = self
                    .rows
                    .get_mut((area.y + row) as usize)
                    .unwrap()
                    .get_mut((area.x + column) as usize)
                    .unwrap();

                cell.background_color = T.bg;
                cell.char = ' ';
            }
        }
    }
}
