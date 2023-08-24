use std::{io::{Stdout, Write}, fs::File};

use crossterm::{
    execute,
    cursor::MoveTo, terminal::{Clear, ClearType},
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read, KeyEventState},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode, size},
};

use crate::{window::{Window, Size}, buffer::BufferMode};

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
        let cursor_position = self.window.get_active_buffer_cursor_position();
        self.move_to(cursor_position.y, cursor_position.x + 4);
    }

    pub fn redraw_statusbar(&mut self) {
        self.move_to(self.window.position.y + self.window.size.height + 1, self.window.position.x);
        let active_buffer = self.window.get_active_buffer();
        print!("{} ({}x{})", active_buffer.mode, active_buffer.cursor.column + 1, active_buffer.cursor.row + 1);
        self.stdout.flush().unwrap();
    }

    pub fn redraw_all(&mut self) {
        self.clear_all();
        for (i, line) in self.window.get_active_buffer().lines.iter().enumerate() {
            self.move_to(self.window.position.y + i as u16, self.window.position.x);
            println!("{:^3} {}", i + 1, line);
        }
        self.redraw_statusbar();
        self.move_to_cursor();
    }

    pub fn get_terminal_size() -> Size {
        let (columns, rows) = size().unwrap();
        Size { width: columns, height: rows }
    }

    pub fn read(&mut self) -> Option<KeyEvent> {
        let event = read().unwrap();

        match event {
            Event::Key(key_event) => Some(key_event),
            Event::Resize(columns, rows) => {
                self.window.size.width = columns;
                self.window.size.height = rows - 2;
                self.redraw_all();
                None
            },
            _ => None
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
            _ => {},
        };
    }

    pub fn handle_key_press_insert(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(ch) => self.window.get_active_buffer_mut().insert_char(ch),
            KeyCode::Tab => {
                self.window.get_active_buffer_mut().insert_char(' ');
                self.window.get_active_buffer_mut().insert_char(' ');
            },
            KeyCode::Enter => self.window.get_active_buffer_mut().insert_newline(),

            KeyCode::Backspace => self.window.get_active_buffer_mut().delete_char(),
            KeyCode::Delete => {},

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
