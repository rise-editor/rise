use std::collections::HashMap;

use crate::buffer::ActionMap;

macro_rules! buffer_action {
    ($action:ident) => {
        |editor| {
            let buffer = editor.get_active_buffer_or_popup_mut();
            buffer.$action()
        }
    };
}

pub fn get_default_normal_maps() -> ActionMap {
    let mut map: ActionMap = HashMap::new();

    // movements
    map.insert("left", buffer_action!(move_left));
    map.insert("down", buffer_action!(move_down));
    map.insert("up", buffer_action!(move_up));
    map.insert("right", buffer_action!(move_right));

    map.insert("h", buffer_action!(move_left));
    map.insert("j", buffer_action!(move_down));
    map.insert("k", buffer_action!(move_up));
    map.insert("l", buffer_action!(move_right));

    map.insert("0", buffer_action!(move_first_column));
    map.insert("$", buffer_action!(move_last_column));
    map.insert("g", buffer_action!(move_first_line));
    map.insert("G", buffer_action!(move_last_line));

    map.insert("w", buffer_action!(move_next_word));
    map.insert("e", buffer_action!(move_next_word_end));
    map.insert("b", buffer_action!(move_previous_word));

    map.insert("<c-y>", buffer_action!(scroll_up));
    map.insert("<c-u>", buffer_action!(scroll_up_half_page));
    map.insert("<c-e>", buffer_action!(scroll_down));
    map.insert("<c-d>", buffer_action!(scroll_down_half_page));

    map.insert("n", buffer_action!(move_to_next_find));
    map.insert("N", buffer_action!(move_to_previous_find));

    // operations
    map.insert("s", buffer_action!(substitute_char));
    map.insert("x", buffer_action!(delete_char));
    map.insert("J", buffer_action!(join_lines_cursor));
    map.insert("O", buffer_action!(open_new_line_previous));
    map.insert("o", buffer_action!(open_new_line_next));

    // modes
    map.insert("i", buffer_action!(enter_insert_mode));
    map.insert("I", buffer_action!(enter_insert_mode_start));
    map.insert("a", buffer_action!(enter_insert_mode_after));
    map.insert("A", buffer_action!(enter_insert_mode_end));
    map.insert(":", buffer_action!(enter_command_mode));
    map.insert("v", buffer_action!(enter_visual_mode));
    map.insert("/", buffer_action!(enter_find_mode));

    map
}

pub fn get_default_visual_maps() -> ActionMap {
    let mut map: ActionMap = HashMap::new();

    map.insert("esc", buffer_action!(enter_normal_mode));

    map.insert("left", buffer_action!(move_left));
    map.insert("down", buffer_action!(move_down));
    map.insert("up", buffer_action!(move_up));
    map.insert("right", buffer_action!(move_right));

    map.insert("h", buffer_action!(move_left));
    map.insert("j", buffer_action!(move_down));
    map.insert("k", buffer_action!(move_up));
    map.insert("l", buffer_action!(move_right));

    map.insert("o", buffer_action!(reverse_selection));

    map
}

pub fn get_default_insert_maps() -> ActionMap {
    let mut map: ActionMap = HashMap::new();

    map.insert("esc", buffer_action!(enter_normal_mode));
    map.insert("enter", buffer_action!(split_line_cursor));
    map.insert("backspace", buffer_action!(delete_char_before_cursor));
    map.insert("delete", buffer_action!(delete_char));
    map.insert("<c-j>", buffer_action!(split_line_cursor));

    map
}
