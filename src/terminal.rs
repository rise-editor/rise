use std::{
    fs::File,
    io::{Stdout, Write},
};

use crossterm::{
    cursor::MoveTo,
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
        let x = self.window.get_active_buffer_visible_x(self.window.get_active_buffer().cursor.x);
        let y = self.window.get_active_buffer_visible_y(self.window.get_active_buffer().cursor.y);

        self.move_to(y, x + 4);
    }

    pub fn redraw_statusbar(&mut self) {
        self.move_to(
            self.window.position.y + self.window.size.height,
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
        let line_count = buffer.get_line_count();

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
        self.redraw_statusbar();
        self.move_to_cursor();
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

    pub fn handle_key_press_normal(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Left => self.window.get_active_buffer_mut().move_left(),
            KeyCode::Right => self.window.get_active_buffer_mut().move_right(),
            KeyCode::Up => self.window.get_active_buffer_mut().move_up(),
            KeyCode::Down => self.window.get_active_buffer_mut().move_down(),

            KeyCode::Char('h') => self.window.get_active_buffer_mut().move_left(),
            KeyCode::Char('j') => self.window.get_active_buffer_mut().move_down(),
            KeyCode::Char('k') => self.window.get_active_buffer_mut().move_up(),
            KeyCode::Char('l') => self.window.get_active_buffer_mut().move_right(),

            KeyCode::Char('i') => self.window.get_active_buffer_mut().enter_insert_mode(),

            KeyCode::Esc => self.stop_requested = true,
            _ => {}
        };
    }

    pub fn handle_key_press_insert(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(ch) => self.window.get_active_buffer_mut().insert_char(ch),
            KeyCode::Tab => {
                self.window.get_active_buffer_mut().insert_char(' ');
                self.window.get_active_buffer_mut().insert_char(' ');
            }
            KeyCode::Enter => self.window.get_active_buffer_mut().insert_newline(),

            KeyCode::Backspace => self.window.get_active_buffer_mut().delete_char(),
            KeyCode::Delete => {}

            KeyCode::Left => self.window.get_active_buffer_mut().move_left(),
            KeyCode::Right => self.window.get_active_buffer_mut().move_right(),
            KeyCode::Up => self.window.get_active_buffer_mut().move_up(),
            KeyCode::Down => self.window.get_active_buffer_mut().move_down(),

            KeyCode::Esc => self.window.get_active_buffer_mut().enter_normal_mode(),
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
            _ => todo!(),
        }

        self.redraw_all();
    }
}
