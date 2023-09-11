pub mod buffer;
pub mod commands;
pub mod core;
pub mod terminal;
pub mod window;

use std::io::stdout;

use crate::buffer::Buffer;
use crate::core::{Point, Size};
use crate::terminal::Terminal;
use crate::window::Window;

fn main() {
    let stdout = stdout();

    let terminal_size = Terminal::get_terminal_size();

    let mut window = Window {
        position: Point { x: 5, y: 5 },
        size: Size {
            width: terminal_size.width / 4,
            height: terminal_size.height / 2 - 2,
        },
        buffers: vec![],
    };

    let buffer = Buffer::new(Size {
        width: terminal_size.width / 4,
        height: terminal_size.height / 2 - 2,
    });

    window.buffers.push(buffer);

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
}
