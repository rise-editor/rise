use std::io::{stdout, Result, Stdout, Write};

use crossterm::{cursor, event, style, terminal, QueueableCommand};

use crate::{
    core::{key::Key, Size},
    screen::Screen,
};

pub struct Terminal {
    pub stdout: Stdout,
    pub screen: Screen,
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

        let size = Terminal::get_terminal_size().unwrap();

        Terminal {
            stdout,
            screen: Screen::new(size.height, size.width),
        }
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
        crossterm::queue!(&self.stdout, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        self.flush()?;
        Ok(())
    }

    pub fn terminate(&mut self) -> Result<()> {
        crossterm::queue!(&self.stdout, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        self.flush()?;
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

    pub fn redraw(&mut self, new_screen: Screen, force: bool) -> Result<()> {
        self.set_cursor_style(new_screen.cursor_style.clone())?;

        let mut bg = style::Color::Black;
        let mut fg = style::Color::White;
        let mut skipped = true;

        for row in 0..new_screen.size.height {
            for column in 0..new_screen.size.width {
                let cell_new = new_screen.cell(row, column).unwrap();
                let cell_old_option = self.screen.cell(row, column);

                if !force {
                    if let Some(cell_old) = cell_old_option {
                        if cell_old == cell_new {
                            skipped = true;
                            continue;
                        }
                    }
                }

                if skipped {
                    self.move_to(row, column)?;
                }

                let cell_bg = style::Color::Rgb {
                    r: cell_new.background_color.0,
                    g: cell_new.background_color.1,
                    b: cell_new.background_color.2,
                };

                if bg != cell_bg {
                    bg = cell_bg;
                    self.stdout.queue(style::SetBackgroundColor(bg))?;
                }

                let cell_fg = style::Color::Rgb {
                    r: cell_new.color.0,
                    g: cell_new.color.1,
                    b: cell_new.color.2,
                };

                if fg != cell_fg {
                    fg = cell_fg;
                    self.stdout.queue(style::SetForegroundColor(fg))?;
                }

                self.stdout.queue(style::Print(cell_new.char))?;
            }

            skipped = true;
        }

        self.move_to(new_screen.cursor.y, new_screen.cursor.x)?;

        self.flush()?;

        self.screen = new_screen;

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
