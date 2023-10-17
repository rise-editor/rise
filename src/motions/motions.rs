use crate::{
    buffer::Buffer,
    core::Point,
    motions::content_reader::{CharType, ContentReader, Direction},
};

pub fn get_word_end_position(buffer: &mut Buffer) -> Point<usize> {
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
