use crate::{
    buffer::Buffer,
    core::point::Point,
    motions::content_reader::{CharType, ContentReader, Direction},
};

pub fn get_next_word_start_position(buffer: &mut Buffer) -> Point<usize> {
    let mut lines: Vec<&str> = vec![];

    for line in buffer.lines.iter() {
        lines.push(line.as_str());
    }

    let mut reader = ContentReader {
        position: buffer.cursor.clone(),
        lines,
    };

    let char_type = reader.get_char_type();

    if char_type != CharType::Whitespace {
        reader.move_while(char_type.clone(), Direction::Forward);

        let char_type_new = reader.get_char_type();

        // end of line
        if char_type == char_type_new {
            reader.move_forward();
        }
    }

    reader.move_while(CharType::Whitespace, Direction::Forward);

    reader.position
}

pub fn get_next_word_end_position(buffer: &mut Buffer) -> Point<usize> {
    let mut lines: Vec<&str> = vec![];

    for line in buffer.lines.iter() {
        lines.push(line.as_str());
    }

    let mut reader = ContentReader {
        position: buffer.cursor.clone(),
        lines,
    };

    reader.move_forward();
    reader.move_while(CharType::Whitespace, Direction::Forward);

    if !reader.is_file_last_char() {
        let char_type = reader.get_char_type();
        reader.move_while(char_type.clone(), Direction::Forward);
        if reader.get_char_type() != char_type {
            reader.move_backward();
        }
    }

    reader.position
}

pub fn get_previous_word_start_position(buffer: &mut Buffer) -> Point<usize> {
    let mut lines: Vec<&str> = vec![];

    for line in buffer.lines.iter() {
        lines.push(line.as_str());
    }

    let mut reader = ContentReader {
        position: buffer.cursor.clone(),
        lines,
    };

    reader.move_backward();
    reader.move_while(CharType::Whitespace, Direction::Backward);

    if !reader.is_file_first_char() {
        let char_type = reader.get_char_type();
        reader.move_while(char_type.clone(), Direction::Backward);
        if reader.get_char_type() != char_type {
            reader.move_forward();
        }
    }

    reader.position
}
