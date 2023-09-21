use std::io::{stdout, Result, Stdout, Write};

use crossterm::{cursor, event, style, terminal};

use crate::{
    core::{key::Key, Size},
    palette::Palette,
};

pub struct Terminal {
    pub stdout: Stdout,
}

#[derive(Clone)]
pub enum CursorStyle {
    BlinkingBar,
    BlinkingBlock,
}

pub enum TerminalEvent {
    Resize(Size<u16>),
    Key(Key),
}

impl Terminal {
    pub fn new() -> Self {
        let stdout = stdout();

        Terminal { stdout }
    }

    pub fn get_terminal_size() -> Result<Size<u16>> {
        let (columns, rows) = terminal::size()?;

        Ok(Size {
            width: columns,
            height: rows,
        })
    }
}

impl Terminal {
    pub fn initialize(&mut self) -> Result<()> {
        crossterm::execute!(&self.stdout, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        Ok(())
    }

    pub fn terminate(&mut self) -> Result<()> {
        crossterm::execute!(&self.stdout, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn set_cursor_style(&mut self, cursor_style: CursorStyle) -> Result<()> {
        match cursor_style {
            CursorStyle::BlinkingBar => {
                crossterm::queue!(&self.stdout, cursor::SetCursorStyle::BlinkingBar)
            }
            CursorStyle::BlinkingBlock => {
                crossterm::queue!(&self.stdout, cursor::SetCursorStyle::BlinkingBlock)
            }
        }
    }

    pub fn redraw(&mut self, palette: &Palette) -> Result<()> {
        self.clear()?;
        self.move_to(0, 0)?;

        self.set_cursor_style(palette.cursor_style.clone())?;

        for row in 0..palette.size.height {
            for column in 0..palette.size.width {
                let cell = palette
                    .rows
                    .get(row as usize)
                    .unwrap()
                    .get(column as usize)
                    .unwrap();

                crossterm::queue!(
                    &self.stdout,
                    style::SetBackgroundColor(style::Color::Rgb {
                        r: cell.background_color.0,
                        g: cell.background_color.1,
                        b: cell.background_color.2,
                    }),
                    style::SetForegroundColor(style::Color::Rgb {
                        r: cell.color.0,
                        g: cell.color.1,
                        b: cell.color.2,
                    }),
                    style::Print(cell.char)
                )?;
            }
        }

        self.move_to(palette.cursor.y, palette.cursor.x)?;

        self.flush()?;

        Ok(())
    }

    pub fn move_to(&mut self, row: u16, column: u16) -> Result<()> {
        crossterm::queue!(&self.stdout, cursor::MoveTo(column, row))
    }

    pub fn clear(&mut self) -> Result<()> {
        crossterm::queue!(&self.stdout, terminal::Clear(terminal::ClearType::All))
    }

    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }

    pub fn read(&self) -> Result<TerminalEvent> {
        let event = event::read()?;

        match event {
            event::Event::Key(key_event) => Ok(TerminalEvent::Key(key_event_to_key(key_event))),
            event::Event::Resize(columns, rows) => Ok(TerminalEvent::Resize(Size {
                width: columns,
                height: rows,
            })),
            _ => todo!(),
        }
    }
}

fn key_event_to_key(key_event: event::KeyEvent) -> Key {
    let code = match key_event.code {
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

    Key {
        ctrl: key_event.modifiers.contains(event::KeyModifiers::CONTROL),
        win: key_event.modifiers.contains(event::KeyModifiers::META),
        alt: key_event.modifiers.contains(event::KeyModifiers::ALT),
        code,
    }
}
