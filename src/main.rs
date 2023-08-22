pub mod cursor;
pub mod buffer;
pub mod terminal;

use std::io::stdout;

use cursor::Cursor;
use buffer::Buffer;
use terminal::Terminal;

fn main() {
    let stdout = stdout();

    let buffer = Buffer {
        cursor: Cursor{ row: 0, column: 0 },
        lines: vec![String::new()],
    };

    let mut terminal = Terminal {
        stdout,
        buffer,
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
