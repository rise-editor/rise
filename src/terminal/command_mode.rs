use crate::terminal::Terminal;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_press_command(terminal: &mut Terminal, event: KeyEvent) {
    let buffer = terminal.window.get_active_buffer_mut();

    match event.code {
        KeyCode::Char(ch) => buffer.command_line.insert_key(ch),
        KeyCode::Enter => buffer.run_command(),

        KeyCode::Backspace => buffer.command_line.delete_key(),
        KeyCode::Delete => buffer.command_line.delete_key(),

        KeyCode::Left => buffer.command_line.move_left(),
        KeyCode::Right => buffer.command_line.move_right(),

        KeyCode::Esc => terminal.enter_normal_mode(),
        _ => todo!(),
    };
}
