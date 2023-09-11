use crate::terminal::Terminal;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_press_visual(terminal: &mut Terminal, event: KeyEvent) {
    let buffer = terminal.window.get_active_buffer_mut();

    match event.code {
        KeyCode::Esc => terminal.enter_normal_mode(),

        KeyCode::Char('h') => buffer.move_left(),
        KeyCode::Char('j') => buffer.move_down(),
        KeyCode::Char('k') => buffer.move_up(),
        KeyCode::Char('l') => buffer.move_right(),

        KeyCode::Char('o') => buffer.reverse_selection(),

        _ => todo!(),
    };
}
