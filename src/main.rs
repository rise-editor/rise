pub mod buffer;
pub mod commands;
pub mod core;
pub mod editor;
pub mod motions;
pub mod screen;
pub mod tab;
pub mod terminal;
pub mod theme;

use std::io::Result;

use crate::{
    core::rectangle::Rectangle,
    editor::Editor,
    screen::Screen,
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

    let screen = Screen::from(&editor);
    terminal.redraw(screen, true)?;

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

        let screen = Screen::from(&editor);
        terminal.redraw(screen, false)?;
    }

    terminal.terminate()?;

    Ok(())
}
