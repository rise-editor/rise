use std::{io::{Stdout, Write}, fs::File};

use crossterm::{
    execute,
    cursor::MoveTo, terminal::{Clear, ClearType},
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read, KeyEventState},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
};

use crate::buffer::Buffer;

pub struct Terminal {
    pub stdout: Stdout,
    pub buffer: Buffer,
    pub stop_requested: bool,
}

impl Terminal {
    pub fn start(&mut self) {
        enable_raw_mode().unwrap();
        execute!(self.stdout, EnterAlternateScreen).unwrap();
        self.clear_all();
        self.move_to_cursor();
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
        self.move_to(self.buffer.cursor.row as u16, self.buffer.cursor.column as u16);
    }

    pub fn redraw_all(&mut self) {
        self.clear_all();
        for (i, line) in self.buffer.lines.iter().enumerate() {
            self.move_to(i as u16, 0);
            println!("{}", line);
        }
        self.move_to_cursor();
    }

    pub fn read(&self) -> Option<KeyEvent> {
        let event = read().unwrap();

        if let Event::Key(key_event) = event {
            Some(key_event)
        } else {
            None
        }
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
            file.write_all(self.buffer.get_content().as_bytes()).unwrap();
            return;
        }

        match event.code {
            KeyCode::Char(ch) => self.buffer.insert_char(ch),
            KeyCode::Tab => {
                self.buffer.insert_char(' ');
                self.buffer.insert_char(' ');
            },
            KeyCode::Enter => self.buffer.insert_newline(),

            KeyCode::Backspace => self.buffer.delete_char(),
            KeyCode::Delete => {},

            KeyCode::Left => self.buffer.move_left(),
            KeyCode::Right => self.buffer.move_right(),
            KeyCode::Up => self.buffer.move_up(),
            KeyCode::Down => self.buffer.move_down(),

            KeyCode::Esc => self.stop_requested = true,
            _ => todo!(),
        };

        self.redraw_all();
    }
}
