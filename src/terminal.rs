use std::io::{Result, Stdout, Write};

use crossterm::{cursor, event, execute, terminal};

use crate::{
    buffer::mode::BufferMode,
    core::{key::Key, Size},
    window::Window,
};

pub struct Terminal {
    pub stdout: Stdout,
    pub window: Window,
    pub stop_requested: bool,
}

impl Terminal {
    pub fn start(&mut self) {
        terminal::enable_raw_mode().unwrap();
        execute!(self.stdout, terminal::EnterAlternateScreen).unwrap();
        self.redraw_all();
        self.stop_requested = false;
    }

    pub fn end(&mut self) {
        terminal::disable_raw_mode().unwrap();
        execute!(self.stdout, terminal::LeaveAlternateScreen).unwrap();
        self.stop_requested = true;
    }

    pub fn clear_all(&self) {
        execute!(&self.stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    pub fn move_to(&self, row: u16, column: u16) {
        execute!(&self.stdout, cursor::MoveTo(column, row)).unwrap();
    }

    pub fn move_to_cursor(&self) {
        let buffer = self.window.get_active_buffer();

        let x;
        let y;

        if buffer.mode == BufferMode::Command {
            x = self.window.position.x + buffer.command.cursor_x as u16 + 1; // TODO: Make command line scrollable
            y = self.window.position.y + self.window.size.height - 2;
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
            print!(":{}", self.window.get_active_buffer().command.text);
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
            active_buffer.area.width,
            active_buffer.area.height
        );
        self.stdout.flush().unwrap();
    }

    pub fn redraw_all(&mut self) {
        match self.window.get_active_buffer().mode {
            BufferMode::Normal => self.set_cursor_blinking_block(),
            BufferMode::Visual => self.set_cursor_blinking_block(),
            BufferMode::Command => self.set_cursor_blinking_bar(),
            BufferMode::Insert => self.set_cursor_blinking_bar(),
        }

        let buffer = self.window.get_active_buffer();
        let line_count = buffer.get_row_count();

        self.clear_all();
        for line_index in buffer.scroll.y..(buffer.scroll.y + buffer.area.height as usize) {
            let x = self.window.get_active_buffer_visible_x(buffer.scroll.x);
            let y = self.window.get_active_buffer_visible_y(line_index);
            self.move_to(y, x);

            if line_index < line_count {
                let line = buffer.get_line_visible_text(line_index);
                println!("{:>3}|{}", line_index + 1, line);
            } else {
                println!("~");
            }
        }
        self.redraw_command_line();
        self.redraw_statusbar();
        self.move_to_cursor();
    }

    pub fn set_cursor_blinking_block(&mut self) {
        execute!(self.stdout, cursor::SetCursorStyle::BlinkingBlock).unwrap();
    }

    pub fn set_cursor_blinking_bar(&mut self) {
        execute!(self.stdout, cursor::SetCursorStyle::BlinkingBar).unwrap();
    }

    pub fn get_terminal_size() -> Result<Size<u16>> {
        let (columns, rows) = terminal::size()?;

        Ok(Size {
            width: columns,
            height: rows,
        })
    }

    pub fn read(&mut self) -> Option<event::KeyEvent> {
        // TODO: log
        let event = event::read().unwrap();

        match event {
            event::Event::Key(key_event) => Some(key_event),
            event::Event::Resize(columns, rows) => {
                self.window.set_size(columns, rows - 2);
                self.redraw_all();
                None
            }
            _ => None,
        }
    }

    pub fn enter_normal_mode(&mut self) {
        self.set_cursor_blinking_block();
    }

    pub fn enter_visual_mode(&mut self) {
        self.window.get_active_buffer_mut().enter_visual_mode();
    }

    pub fn enter_insert_mode(&mut self) {
        self.set_cursor_blinking_bar();
    }

    pub fn handle_key_press(&mut self, event: event::KeyEvent) {
        let code = match event.code {
            event::KeyCode::Char(c) => c.to_string(),
            event::KeyCode::Esc => String::from("esc"),
            event::KeyCode::Tab => String::from("tab"),
            event::KeyCode::Enter => String::from("enter"),
            event::KeyCode::Backspace => String::from("backspace"),
            event::KeyCode::Delete => String::from("delete"),
            event::KeyCode::Up => String::from("up"),
            event::KeyCode::Down => String::from("down"),
            event::KeyCode::Left => String::from("left"),
            event::KeyCode::Right => String::from("right"),
            _ => todo!(),
        };

        if code == "esc" && self.window.get_active_buffer().mode == BufferMode::Normal {
            self.stop_requested = true;
            return;
        }

        let key = Key {
            ctrl: event.modifiers.contains(event::KeyModifiers::CONTROL),
            win: event.modifiers.contains(event::KeyModifiers::META),
            alt: event.modifiers.contains(event::KeyModifiers::ALT),
            code,
        };

        self.window.get_active_buffer_mut().handle_key(key);
        self.redraw_all();
    }
}
