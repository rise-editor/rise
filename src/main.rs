pub mod buffer;
pub mod commands;
pub mod core;
pub mod plugins;
pub mod terminal;
pub mod window;

use std::io::{stdout, Result};

use plugins::explorer::explorer_buffer::create_explorer_buffer;

use crate::core::Size;
use crate::terminal::Terminal;
use crate::window::Window;

fn main() -> Result<()> {
    let stdout = stdout();

    let terminal_size = Terminal::get_terminal_size()?;

    let s = Size {
        width: terminal_size.width,
        height: terminal_size.height - 2,
    };

    let mut window = Window::new(terminal_size);
    let cur_dir = std::env::current_dir().unwrap().display().to_string();
    let explorer_buf = create_explorer_buffer(cur_dir, s);
    window.buffers.push(explorer_buf);
    // window.create_new_buffer();

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
