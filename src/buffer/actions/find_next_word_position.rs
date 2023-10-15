use crate::{buffer::Buffer, core::Point};

#[derive(PartialEq)]
enum CharType {
    Whitespace,
    Word,
    Special,
}

fn get_char_type(ch: char) -> CharType {
    if ch.is_whitespace() {
        CharType::Whitespace
    } else if ch.is_alphanumeric() {
        CharType::Word
    } else {
        CharType::Special
    }
}

pub fn find_next_word_position(buffer: &Buffer) -> Point<usize> {
    let current_line = buffer.get_current_line();
    let mut initial_char_type = CharType::Whitespace;

    if current_line.len() > 0 {
        let cursor_char = current_line.chars().nth(buffer.cursor.x).unwrap();
        initial_char_type = get_char_type(cursor_char);
    }

    for row in buffer.cursor.y..buffer.get_row_count() {
        let mut iter = buffer.get_line(row).chars();
        let mut column = 0;

        if row == buffer.cursor.y {
            iter.nth(buffer.cursor.x);
            column = buffer.cursor.x + 1;
        }

        while let Some(c) = iter.next() {
            let char_type = get_char_type(c);

            if char_type == CharType::Whitespace {
                initial_char_type = CharType::Whitespace;
            } else if char_type != initial_char_type {
                return Point { y: row, x: column };
            }

            column += 1;
        }

        initial_char_type = CharType::Whitespace;
    }

    let last_row = buffer.get_row_count() - 1;

    Point {
        y: last_row,
        x: buffer.get_line_max_cursor_x(last_row),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        buffer::{actions::find_next_word_position::find_next_word_position, Buffer},
        core::Rectangle,
    };

    #[test]
    fn test() {
        let mut area = Rectangle::<u16>::zero();
        area.width = 10;
        area.height = 10;
        let mut buffer = Buffer::new(area);

        let mut position = find_next_word_position(&buffer);

        assert_eq!(0, position.y);
        assert_eq!(0, position.x);

        buffer.set_content(String::from("abc\n   def"));
        buffer.move_cursor(1, 0);

        position = find_next_word_position(&buffer);

        assert_eq!(1, position.y);
        assert_eq!(3, position.x);
    }
}
