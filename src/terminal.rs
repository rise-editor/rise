pub mod command_mode;
pub mod insert_mode;
pub mod normal_mode;
pub mod visual_mode;

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

use crate::{
    buffer::mode::BufferMode, core::Size, window::Window,
    terminal::{
        command_mode::handle_key_press_command,
        insert_mode::handle_key_press_insert,
        normal_mode::handle_key_press_normal,
        visual_mode::handle_key_press_visual,
    }
};

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
            x = self.window.position.x + buffer.command.cursor_x as u16 + 1; // TODO: Make command line scrollable
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

    pub fn enter_visual_mode(&mut self) {
        self.window.get_active_buffer_mut().enter_visual_mode();
    }

    pub fn enter_insert_mode(&mut self) {
        self.window.get_active_buffer_mut().enter_insert_mode();
        self.set_cursor_blinking_bar();
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
            BufferMode::Normal => handle_key_press_normal(self, event),
            BufferMode::Insert => handle_key_press_insert(self, event),
            BufferMode::Command => handle_key_press_command(self, event),
            BufferMode::Visual => handle_key_press_visual(self, event),
        }

        self.redraw_all();
    }
}
