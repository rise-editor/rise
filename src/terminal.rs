use std::{
    fs::File,
    io::{Stdout, Write},
};

use crossterm::{
    cursor::{MoveTo, SetCursorStyle},
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
    terminal::{Clear, ClearType},
};

use crate::{buffer::mode::BufferMode, core::Size, window::Window};

pub struct Terminal {
    pub stdout: Stdout,
    pub window: Window,
    pub stop_requested: bool,
}

impl Terminal {
    pub fn start(&mut self) {
        enable_raw_mode().unwrap();
        execute!(self.stdout, EnterAlternateScreen).unwrap();
        self.redraw_all();
        self.stop_requested = false;
    }

    pub fn end(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
        self.stop_requested = true;
    }

    pub fn clear_all(&self) {
        execute!(&self.stdout, Clear(ClearType::All)).unwrap();
    }

    pub fn move_to(&self, row: u16, column: u16) {
        execute!(&self.stdout, MoveTo(column, row)).unwrap();
    }

    pub fn move_to_cursor(&self) {
        let buffer = self.window.get_active_buffer();

        let x;
        let y;

        if buffer.mode == BufferMode::Command {
            x = self.window.position.x + buffer.command_line.cursor_x as u16 + 1; // TODO: Make command line scrollable
            y = self.window.get_active_buffer_visible_y(buffer.visible_area.height as usize - 2);
        } else {
            x = self.window.get_active_buffer_visible_x(buffer.cursor.x) + 4;
            y = self.window.get_active_buffer_visible_y(buffer.cursor.y);
        }

        self.move_to(y, x);
    }

    pub fn redraw_command_line(&mut self) {
        if self.window.get_active_buffer().mode == BufferMode::Command {
            self.move_to(
                self.window.position.y + self.window.size.height - 2,
                self.window.position.x,
            );
            print!(":{}", self.window.get_active_buffer().command_line.text);
        }
    }

    pub fn redraw_statusbar(&mut self) {
        self.move_to(
            self.window.position.y + self.window.size.height - 1,
            self.window.position.x,
        );
        let active_buffer = self.window.get_active_buffer();
        print!(
            "{} ({}x{}) W: {} H: {}",
            active_buffer.mode,
            active_buffer.cursor.x + 1,
            active_buffer.cursor.y + 1,
            active_buffer.visible_area.width,
            active_buffer.visible_area.height
        );
        self.stdout.flush().unwrap();
    }

    pub fn redraw_all(&mut self) {
        let buffer = self.window.get_active_buffer();
        let line_count = buffer.get_row_count();

        self.clear_all();
        for line_index in buffer.scroll.y..(buffer.scroll.y + buffer.visible_area.height as usize) {
            let x = self.window.get_active_buffer_visible_x(buffer.scroll.x);
            let y = self.window.get_active_buffer_visible_y(line_index);
            self.move_to(y, x);

            if line_index < line_count {
                let line = buffer.get_line_visible_text(line_index);
                println!("{:>3} {}", line_index + 1, line);
            } else {
                println!("~");
            }
        }
        self.redraw_command_line();
        self.redraw_statusbar();
        self.move_to_cursor();
    }

    pub fn set_cursor_blinking_block(&mut self) {
        execute!(self.stdout, SetCursorStyle::BlinkingBlock).unwrap();
    }

    pub fn set_cursor_blinking_bar(&mut self) {
        execute!(self.stdout, SetCursorStyle::BlinkingBar).unwrap();
    }

    pub fn get_terminal_size() -> Size<u16> {
        let (columns, rows) = size().unwrap();
        Size {
            width: columns,
            height: rows,
        }
    }

    pub fn read(&mut self) -> Option<KeyEvent> {
        let event = read().unwrap();

        match event {
            Event::Key(key_event) => Some(key_event),
            Event::Resize(columns, rows) => {
                self.window.set_size(columns, rows - 2);
                self.redraw_all();
                None
            }
            _ => None,
        }
    }

    pub fn enter_normal_mode(&mut self) {
        self.window.get_active_buffer_mut().enter_normal_mode();
        self.set_cursor_blinking_block();
    }

    pub fn enter_insert_mode(&mut self) {
        self.window.get_active_buffer_mut().enter_insert_mode();
        self.set_cursor_blinking_bar();
    }

    pub fn enter_insert_mode_after(&mut self) {
        self.window.get_active_buffer_mut().enter_insert_mode_after();
        self.set_cursor_blinking_bar();
    }

    pub fn handle_key_press_normal(&mut self, event: KeyEvent) {
        let buffer = self.window.get_active_buffer_mut();

        match event.code {
            KeyCode::Esc => self.stop_requested = true,

            KeyCode::Left => buffer.move_left(),
            KeyCode::Right => buffer.move_right(),
            KeyCode::Up => buffer.move_up(),
            KeyCode::Down => buffer.move_down(),

            KeyCode::Char('w') => buffer.move_to_next_word_start(),

            KeyCode::Char('h') => buffer.move_left(),
            KeyCode::Char('j') => buffer.move_down(),
            KeyCode::Char('k') => buffer.move_up(),
            KeyCode::Char('l') => buffer.move_right(),

            KeyCode::Char('g') => buffer.move_first_row(),
            KeyCode::Char('G') => buffer.move_last_row(),
            KeyCode::Char('0') => buffer.move_first_column(),
            KeyCode::Char('$') => buffer.move_last_column(),
                 
            KeyCode::Char('a') => self.enter_insert_mode_after(),
            KeyCode::Char('A') => {
                buffer.move_last_column();
                self.enter_insert_mode_after();
            },
            KeyCode::Char('s') => {
                self.enter_insert_mode();
                let b = self.window.get_active_buffer_mut();
                b.delete_char_before(b.cursor.y, b.cursor.y);
            },
            KeyCode::Char('i') => {
                buffer.enter_insert_mode();
                self.set_cursor_blinking_bar();
            },
            KeyCode::Char('o') => {
                buffer.insert_newline(buffer.cursor.y + 1);
                self.enter_insert_mode();
            }
            KeyCode::Char('O') => {
                buffer.insert_newline(buffer.cursor.y);
                self.enter_insert_mode();
            },

            KeyCode::Char('x') => buffer.delete_char(),

            KeyCode::Char(':') => buffer.enter_command_mode(),

            _ => {}
        };
    }

    pub fn handle_key_press_insert(&mut self, event: KeyEvent) {
        let buffer = self.window.get_active_buffer_mut();

        match event.code {
            KeyCode::Char(ch) => buffer.insert_char(ch),
            KeyCode::Tab => {
                buffer.insert_char(' ');
                buffer.insert_char(' ');
            }
            KeyCode::Enter => buffer.split_line(buffer.cursor.y, buffer.cursor.x),

            KeyCode::Backspace => buffer.delete_char_before(buffer.cursor.y, buffer.cursor.x),
            KeyCode::Delete => buffer.delete_char(),

            KeyCode::Left => buffer.move_left(),
            KeyCode::Right => buffer.move_right(),
            KeyCode::Up => buffer.move_up(),
            KeyCode::Down => buffer.move_down(),

            KeyCode::Esc => self.enter_normal_mode(),
            _ => todo!(),
        };
    }

    pub fn handle_key_press_command(&mut self, event: KeyEvent) {
        let buffer = self.window.get_active_buffer_mut();

        match event.code {
            KeyCode::Char(ch) => buffer.command_line.insert_key(ch),
            KeyCode::Enter => { },

            KeyCode::Backspace => buffer.command_line.delete_key(),
            KeyCode::Delete => buffer.command_line.delete_key(),

            KeyCode::Left => buffer.command_line.move_left(),
            KeyCode::Right => buffer.command_line.move_right(),

            KeyCode::Esc => self.enter_normal_mode(),
            _ => todo!(),
        };
    }

    pub fn handle_key_press(&mut self, event: KeyEvent) {
        let save_key_event = KeyEvent {
            modifiers: KeyModifiers::CONTROL,
            code: KeyCode::Char('s'),
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };

        if event == save_key_event {
            let mut file = File::create("/tmp/editorise.txt").unwrap();
            file.write_all(self.window.get_active_buffer().get_content().as_bytes()).unwrap();
            return;
        }

        match self.window.get_active_buffer().mode {
            BufferMode::Normal => self.handle_key_press_normal(event),
            BufferMode::Insert => self.handle_key_press_insert(event),
            BufferMode::Command => self.handle_key_press_command(event),
            _ => todo!(),
        }

        self.redraw_all();
    }
}
