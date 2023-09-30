use std::collections::HashMap;

use crate::{
    buffer::actions::find_next_word_position::find_next_word_position, core::Size,
    plugins::explorer::explorer_buffer::create_explorer_buffer,
};

use super::ActionMap;

pub fn get_default_insert_maps() -> ActionMap {
    let mut map: ActionMap = HashMap::new();
    map.insert("esc", |editor| {
        editor.get_active_buffer_mut().enter_normal_mode()
    });
    map.insert("enter", |editor| {
        let buffer = editor.get_active_buffer_mut();
        buffer.split_line(buffer.cursor.y, buffer.cursor.x)
    });
    map.insert("backspace", |editor| {
        let buffer = editor.get_active_buffer_mut();
        buffer.delete_char_before(buffer.cursor.y, buffer.cursor.x)
    });
    map.insert("delete", |editor| {
        editor.get_active_buffer_mut().delete_char()
    });
    map.insert("<c-j>", |editor| {
        let buffer = editor.get_active_buffer_mut();
        buffer.split_line(buffer.cursor.y, buffer.cursor.x);
    });
    return map;
}

pub fn get_default_normal_maps() -> ActionMap {
    let mut map: ActionMap = HashMap::new();
    map.insert("left", |editor| editor.get_active_buffer_mut().move_left());
    map.insert("down", |editor| editor.get_active_buffer_mut().move_down());
    map.insert("up", |editor| editor.get_active_buffer_mut().move_up());
    map.insert("right", |editor| {
        editor.get_active_buffer_mut().move_right()
    });

    map.insert("h", |editor| editor.get_active_buffer_mut().move_left());
    map.insert("j", |editor| editor.get_active_buffer_mut().move_down());
    map.insert("k", |editor| editor.get_active_buffer_mut().move_up());
    map.insert("l", |editor| editor.get_active_buffer_mut().move_right());

    map.insert("g", |editor| {
        editor.get_active_buffer_mut().move_first_row()
    });
    map.insert("G", |editor| editor.get_active_buffer_mut().move_last_row());
    map.insert("0", |editor| {
        editor.get_active_buffer_mut().move_first_column()
    });
    map.insert("$", |editor| {
        editor.get_active_buffer_mut().move_last_column()
    });

    map.insert("w", |editor| {
        let buffer = editor.get_active_buffer_mut();
        let new_position = find_next_word_position(buffer);
        buffer.move_cursor(new_position.y, new_position.x);
    });

    map.insert("x", |editor| editor.get_active_buffer_mut().delete_char());

    map.insert("I", |editor| {
        let buffer = editor.get_active_buffer_mut();
        buffer.move_first_column();
        buffer.enter_insert_mode();
    });
    map.insert("i", |editor| {
        editor.get_active_buffer_mut().enter_insert_mode()
    });
    map.insert("A", |editor| {
        let buffer = editor.get_active_buffer_mut();
        buffer.move_last_column();
        buffer.enter_insert_mode();
        buffer.move_right();
    });
    map.insert("a", |editor| {
        let buffer = editor.get_active_buffer_mut();
        buffer.enter_insert_mode();
        buffer.move_right();
    });
    map.insert("O", |editor| {
        let buffer = editor.get_active_buffer_mut();
        buffer.insert_newline(buffer.cursor.y);
        buffer.enter_insert_mode();
    });
    map.insert("o", |editor| {
        let buffer = editor.get_active_buffer_mut();
        buffer.insert_newline(buffer.cursor.y + 1);
        buffer.enter_insert_mode();
    });
    map.insert(":", |editor| {
        editor.get_active_buffer_mut().enter_command_mode()
    });
    map.insert("v", |editor| {
        editor.get_active_buffer_mut().enter_visual_mode()
    });
    return map;
}

pub fn get_default_command_maps() -> ActionMap {
    let mut map: ActionMap = HashMap::new();
    map.insert("esc", |editor| {
        editor.get_active_buffer_mut().enter_normal_mode()
    });
    map.insert("enter", |editor| {
        // TODO: Move this
        let command = editor.get_active_buffer().command.text.trim();
        if command == "e" {
            let window = editor.get_active_window();
            let buffer = create_explorer_buffer(
                String::from("."),
                Size {
                    width: window.size.width,
                    height: window.size.height - 2,
                },
            );
            let window = editor.get_active_window_mut();
            window.buffers.push(buffer);
            window.active_buffer = window.buffers.len() - 1;
        } else {
            editor.get_active_buffer_mut().run_command()
        }
    });
    map.insert("backspace", |editor| {
        editor.get_active_buffer_mut().command.delete_char()
    });
    map.insert("left", |editor| {
        editor.get_active_buffer_mut().command.move_left()
    });
    map.insert("right", |editor| {
        editor.get_active_buffer_mut().command.move_right()
    });
    return map;
}

pub fn get_default_visual_maps() -> ActionMap {
    let mut map: ActionMap = HashMap::new();
    map.insert("esc", |editor| {
        editor.get_active_buffer_mut().enter_normal_mode()
    });

    map.insert("left", |editor| editor.get_active_buffer_mut().move_left());
    map.insert("down", |editor| editor.get_active_buffer_mut().move_down());
    map.insert("up", |editor| editor.get_active_buffer_mut().move_up());
    map.insert("right", |editor| {
        editor.get_active_buffer_mut().move_right()
    });

    map.insert("h", |editor| editor.get_active_buffer_mut().move_left());
    map.insert("j", |editor| editor.get_active_buffer_mut().move_down());
    map.insert("k", |editor| editor.get_active_buffer_mut().move_up());
    map.insert("l", |editor| editor.get_active_buffer_mut().move_right());

    map.insert("o", |editor| {
        editor.get_active_buffer_mut().reverse_selection()
    });

    return map;
}
