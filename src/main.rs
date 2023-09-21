pub mod buffer;
pub mod commands;
pub mod core;
pub mod palette;
pub mod plugins;
pub mod terminal;
pub mod window;

use std::io::Result;

use crate::{
    palette::Palette,
    terminal::{Terminal, TerminalEvent},
    window::Window,
};

fn main() -> Result<()> {
    let terminal_size = Terminal::get_terminal_size()?;

    let mut window = Window::new(terminal_size);
    window.create_new_buffer();

    let mut terminal = Terminal::new();

    terminal.initialize()?;

    let palette = Palette::new(&window);
    terminal.redraw(&palette)?;

    while let Ok(event) = terminal.read() {
        match event {
            TerminalEvent::Resize(size) => window.set_size(size.width, size.height),
            TerminalEvent::Key(key) => {
                if key.ctrl && key.code == String::from("c") {
                    break;
                }
                window.get_active_buffer_mut().handle_key(key);
            }
        }

        let palette = Palette::new(&window);
        terminal.redraw(&palette)?;
    }

    terminal.terminate()?;

    Ok(())
}
