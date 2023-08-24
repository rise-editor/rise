pub mod cursor;
pub mod buffer;
pub mod terminal;
pub mod window;

use std::io::stdout;

use cursor::Cursor;
use buffer::{Buffer, BufferMode};
use terminal::Terminal;
use window::{Window, Size, Point};

fn main() {
    let stdout = stdout();

    let terminal_size = Terminal::get_terminal_size();

    let mut window = Window {
        position: Point {
            x: 0,
            y: 1,
        },
        size: Size {
            width: terminal_size.width,
            height: terminal_size.height - 2,
        },
        buffers: vec![],
    };

    let buffer = Buffer {
        mode: BufferMode::Normal,
        cursor: Cursor{ row: 0, column: 0 },
        lines: vec![String::new()],
    };

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
