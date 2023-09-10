use crate::terminal::Terminal;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_press_insert(terminal: &mut Terminal, event: KeyEvent) {
    let buffer = terminal.window.get_active_buffer_mut();

    match event.code {
        KeyCode::Char(ch) => buffer.insert_char(ch),
        KeyCode::Tab => {
            buffer.insert_char(' ');
            buffer.insert_char(' ');
        }
        KeyCode::Enter => buffer.split_line(buffer.cursor.y, buffer.cursor.x),

        KeyCode::Backspace => buffer.delete_char_before(buffer.cursor.y, buffer.cursor.x),
        KeyCode::Delete => buffer.delete_char(),

        KeyCode::Left => buffer.move_left(),
        KeyCode::Right => buffer.move_right(),
        KeyCode::Up => buffer.move_up(),
        KeyCode::Down => buffer.move_down(),

        KeyCode::Esc => terminal.enter_normal_mode(),
        _ => todo!(),
    };
}
