pub mod buffer;
pub mod commands;
pub mod core;
pub mod terminal;
pub mod window;

use std::io::{stdout, Result};

use crate::terminal::Terminal;
use crate::window::Window;

fn main() -> Result<()> {
    let stdout = stdout();

    let terminal_size = Terminal::get_terminal_size()?;

    let mut window = Window::new(terminal_size);
    window.create_new_buffer();

    let mut terminal = Terminal {
        stdout,
        window,
        stop_requested: true,
    };

    terminal.start();

    while !terminal.stop_requested {
        if let Some(key_event) = terminal.read() {
            terminal.handle_key_press(key_event);
        }
    }

    terminal.end();

    Ok(())
}
