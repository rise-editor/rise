use crate::terminal::Terminal;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_press_normal(terminal: &mut Terminal, event: KeyEvent) {
    let buffer = terminal.window.get_active_buffer_mut();

    match event.code {
        KeyCode::Esc => terminal.stop_requested = true,

        KeyCode::Left => buffer.move_left(),
        KeyCode::Right => buffer.move_right(),
        KeyCode::Up => buffer.move_up(),
        KeyCode::Down => buffer.move_down(),

        KeyCode::Char('w') => buffer.move_to_next_word_start(),

        KeyCode::Char('h') => buffer.move_left(),
        KeyCode::Char('j') => buffer.move_down(),
        KeyCode::Char('k') => buffer.move_up(),
        KeyCode::Char('l') => buffer.move_right(),

        KeyCode::Char('g') => buffer.move_first_row(),
        KeyCode::Char('G') => buffer.move_last_row(),
        KeyCode::Char('0') => buffer.move_first_column(),
        KeyCode::Char('$') => buffer.move_last_column(),

        KeyCode::Char('a') => {
            terminal.enter_insert_mode();
            terminal.window.get_active_buffer_mut().move_right();
        },
        KeyCode::Char('A') => {
            buffer.move_last_column();
            terminal.enter_insert_mode();
            terminal.window.get_active_buffer_mut().move_right();
        }
        KeyCode::Char('s') => {
            terminal.enter_insert_mode();
            let b = terminal.window.get_active_buffer_mut();
            b.delete_char_before(b.cursor.y, b.cursor.y);
        }
        KeyCode::Char('i') => {
            buffer.enter_insert_mode();
            terminal.set_cursor_blinking_bar();
        }
        KeyCode::Char('o') => {
            buffer.insert_newline(buffer.cursor.y + 1);
            terminal.enter_insert_mode();
        }
        KeyCode::Char('O') => {
            buffer.insert_newline(buffer.cursor.y);
            terminal.enter_insert_mode();
        }

        KeyCode::Char('x') => buffer.delete_char(),

        KeyCode::Char('v') => terminal.enter_visual_mode(),

        KeyCode::Char(':') => buffer.enter_command_mode(),

        _ => {}
    };
}
