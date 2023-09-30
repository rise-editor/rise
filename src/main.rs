pub mod buffer;
pub mod commands;
pub mod core;
pub mod editor;
pub mod palette;
pub mod plugins;
pub mod terminal;
pub mod window;

use std::io::Result;

use crate::{
    editor::Editor,
    palette::Palette,
    terminal::{Terminal, TerminalEvent},
};

fn main() -> Result<()> {
    let terminal_size = Terminal::get_terminal_size()?;

    let mut editor = Editor::new(terminal_size);
    let window = editor.create_new_window();
    window.create_new_buffer();

    let mut terminal = Terminal::new();

    terminal.initialize()?;

    let palette = Palette::new(&editor);
    terminal.redraw(&palette)?;

    while let Ok(event) = terminal.read() {
        match event {
            TerminalEvent::Resize(size) => editor.set_size(size.width, size.height),
            TerminalEvent::Key(key) => {
                if key.ctrl && key.code == String::from("c") {
                    break;
                }
                editor.get_active_window_mut().get_active_buffer_mut().handle_key(key);
            }
        }

        let palette = Palette::new(&editor);
        terminal.redraw(&palette)?;
    }

    terminal.terminate()?;

    Ok(())
}
