pub mod buffer;
pub mod commands;
pub mod core;
pub mod editor;
pub mod palette;
pub mod plugins;
pub mod tab;
pub mod terminal;

use std::io::Result;

use crate::core::Rectangle;

use crate::{
    editor::Editor,
    palette::Palette,
    terminal::{Terminal, TerminalEvent},
};

fn main() -> Result<()> {
    let terminal_size = Terminal::get_terminal_size()?;
    let editor_area: Rectangle<u16> = Rectangle::<u16>::from_size(terminal_size);

    let mut editor = Editor::new(editor_area);
    let tab = editor.create_new_tab();
    tab.create_new_buffer();

    let mut terminal = Terminal::new();

    terminal.initialize()?;

    let palette = Palette::from(&editor);
    terminal.redraw(&palette)?;

    while let Ok(event) = terminal.read() {
        match event {
            TerminalEvent::Resize(size) => editor.set_size(Rectangle::<u16>::from_size(size)),
            TerminalEvent::Key(key) => {
                if key.ctrl && key.code == String::from("c") {
                    break;
                }
                editor.handle_key(key);
            }
        }

        let palette = Palette::from(&editor);
        terminal.redraw(&palette)?;
    }

    terminal.terminate()?;

    Ok(())
}
