use std::collections::HashMap;

use crate::buffer::Buffer;

pub fn get_default_insert_maps() -> HashMap<&'static str, fn(&mut Buffer)> {
    let mut map: HashMap<&str, fn(&mut Buffer)> = HashMap::new();
    map.insert("esc", |buffer| buffer.enter_normal_mode());
    map.insert("enter", |buffer| {
        buffer.split_line(buffer.cursor.y, buffer.cursor.x)
    });
    map.insert("backspace", |buffer| {
        buffer.delete_char_before(buffer.cursor.y, buffer.cursor.x)
    });
    map.insert("delete", |buffer| buffer.delete_char());
    map.insert("<c-j>", |buffer| {
        buffer.split_line(buffer.cursor.y, buffer.cursor.x);
    });
    map.insert("<c-/>", |buffer| buffer.enter_command_mode());
    return map;
}

pub fn get_default_normal_maps() -> HashMap<&'static str, fn(&mut Buffer)> {
    let mut map: HashMap<&str, fn(&mut Buffer)> = HashMap::new();
    map.insert("left", |buffer| buffer.move_left());
    map.insert("down", |buffer| buffer.move_down());
    map.insert("up", |buffer| buffer.move_up());
    map.insert("right", |buffer| buffer.move_right());

    map.insert("h", |buffer| buffer.move_left());
    map.insert("j", |buffer| buffer.move_down());
    map.insert("k", |buffer| buffer.move_up());
    map.insert("l", |buffer| buffer.move_right());

    map.insert("g", |buffer| buffer.move_first_row());
    map.insert("G", |buffer| buffer.move_last_row());
    map.insert("0", |buffer| buffer.move_first_column());
    map.insert("$", |buffer| buffer.move_last_column());

    map.insert("w", |buffer| buffer.move_to_next_word_start());

    map.insert("x", |buffer| buffer.delete_char());

    map.insert("I", |buffer| {
        buffer.move_first_column();
        buffer.enter_insert_mode();
    });
    map.insert("i", |buffer| buffer.enter_insert_mode());
    map.insert("A", |buffer| {
        buffer.move_last_column();
        buffer.enter_insert_mode();
        buffer.move_right();
    });
    map.insert("a", |buffer| {
        buffer.enter_insert_mode();
        buffer.move_right();
    });
    map.insert("O", |buffer| {
        buffer.insert_newline(buffer.cursor.y);
        buffer.enter_insert_mode();
    });
    map.insert("o", |buffer| {
        buffer.insert_newline(buffer.cursor.y + 1);
        buffer.enter_insert_mode();
    });
    map.insert(":", |buffer| buffer.enter_command_mode());
    map.insert("v", |buffer| buffer.enter_visual_mode());
    return map;
}

pub fn get_default_command_maps() -> HashMap<&'static str, fn(&mut Buffer)> {
    let mut map: HashMap<&str, fn(&mut Buffer)> = HashMap::new();
    map.insert("esc", |buffer| buffer.enter_normal_mode());
    map.insert("enter", |buffer| buffer.run_command());
    map.insert("backspace", |buffer| buffer.command.delete_key());
    map.insert("left", |buffer| buffer.command.move_left());
    map.insert("right", |buffer| buffer.command.move_right());
    return map;
}

pub fn get_default_visual_maps() -> HashMap<&'static str, fn(&mut Buffer)> {
    let mut map: HashMap<&str, fn(&mut Buffer)> = HashMap::new();
    map.insert("esc", |buffer| buffer.enter_normal_mode());

    map.insert("left", |buffer| buffer.move_left());
    map.insert("down", |buffer| buffer.move_down());
    map.insert("up", |buffer| buffer.move_up());
    map.insert("right", |buffer| buffer.move_right());

    map.insert("h", |buffer| buffer.move_left());
    map.insert("j", |buffer| buffer.move_down());
    map.insert("k", |buffer| buffer.move_up());
    map.insert("l", |buffer| buffer.move_right());

    map.insert("o", |buffer| buffer.reverse_selection());

    return map;
}
