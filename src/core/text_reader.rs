use crate::core::{point::Point, text_position::TextPosition};

pub struct TextReader<'a> {
    cursor: Point<usize>,
    pub lines: &'a Vec<String>,
}

impl<'a> TextReader<'a> {
    pub fn new(lines: &'a Vec<String>) -> Self {
        Self {
            cursor: Point::new(0, 0),
            lines,
        }
    }

    pub fn set_cursor(&mut self, cursor: Point<usize>) -> Result<(), String> {
        if cursor.y < self.lines.len() {
            if cursor.x <= self.get_line_last_x(cursor.y) {
                self.cursor = cursor;
                return Ok(());
            }
        }
        Err(format!("No char at x:{}, y:{}", cursor.x, cursor.y))
    }

    pub fn get_cursor(&self) -> Point<usize> {
        self.cursor.clone()
    }

    pub fn get_cursor_x(&self) -> usize {
        self.cursor.x
    }

    pub fn get_cursor_y(&self) -> usize {
        self.cursor.y
    }

    pub fn get_char(&self) -> Option<char> {
        self.lines.get(self.cursor.y)?.chars().nth(self.cursor.x)
    }

    pub fn get_line_last_x(&self, y: usize) -> usize {
        self.lines.get(y).unwrap().len().checked_sub(1).unwrap_or(0)
    }

    pub fn previous(&mut self) -> Option<char> {
        if self.is_text_first_x() {
            return None;
        } else if self.is_line_first_x() {
            self.cursor.y -= 1;
            self.cursor.x = self.get_line_last_x(self.cursor.y);
        } else {
            self.cursor.x -= 1;
        }
        self.get_char()
    }

    pub fn next(&mut self) -> Option<char> {
        if self.is_text_last_x() {
            return None;
        } else if self.is_line_last_x() {
            self.cursor.y += 1;
            self.cursor.x = 0;
        } else {
            self.cursor.x += 1;
        }
        self.get_char()
    }

    pub fn is_line_first_x(&self) -> bool {
        self.cursor.x == 0
    }

    pub fn is_line_last_x(&self) -> bool {
        self.cursor.x == self.get_line_last_x(self.cursor.y)
    }

    pub fn is_text_first_x(&self) -> bool {
        self.cursor.y == 0 && self.cursor.x == 0
    }

    pub fn is_text_last_x(&self) -> bool {
        self.cursor.y + 1 == self.lines.len() && self.is_line_last_x()
    }

    pub fn get_text_positions(&self, p1: Point<usize>, p2: Point<usize>) -> Vec<TextPosition> {
        let (from, to) = Point::order(p1, p2);

        let mut result: Vec<TextPosition> = Vec::new();

        for y in from.y..(to.y + 1) {
            let x1 = if from.y == y { from.x } else { 0 };
            let x2 = if to.y == y {
                to.x
            } else {
                self.get_line_last_x(y)
            };
            result.push(TextPosition {
                row: y,
                start: x1,
                end: x2,
            });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::core::point::Point;

    use super::TextReader;

    #[test]
    fn test() {
        let lines: Vec<String> = vec![
            String::from("12 3 "),
            String::from(""),
            String::from("abc de"),
            String::from("a"),
        ];
        let mut reader = TextReader::new(&lines);
        assert_eq!(true, reader.is_line_first_x());
        assert_eq!(true, reader.is_text_first_x());
        assert_eq!(false, reader.is_line_last_x());
        assert_eq!(false, reader.is_text_last_x());
        assert_eq!(Some('1'), reader.get_char());
        assert_eq!(Some('2'), reader.next());
        assert_eq!(Some(' '), reader.next());
        assert_eq!(Some('3'), reader.next());
        assert_eq!(Some(' '), reader.next());
        assert_eq!(false, reader.is_line_first_x());
        assert_eq!(false, reader.is_text_first_x());
        assert_eq!(true, reader.is_line_last_x());
        assert_eq!(false, reader.is_text_last_x());
        assert_eq!(None, reader.next());
        assert_eq!(true, reader.is_line_first_x());
        assert_eq!(false, reader.is_text_first_x());
        assert_eq!(true, reader.is_line_last_x());
        assert_eq!(false, reader.is_text_last_x());
        assert_eq!(Ok(()), reader.set_cursor(Point::new(2, 4)));
        assert_eq!(Some('d'), reader.get_char());
        assert_eq!(Some('e'), reader.next());
        assert_eq!(Some('a'), reader.next());
        assert_eq!(true, reader.is_text_last_x());
        assert_eq!(None, reader.next());
        assert_eq!(None, reader.next());
        assert_eq!(None, reader.next());
        assert_eq!(Some('e'), reader.previous());
        assert_eq!(Some('d'), reader.previous());

        assert_eq!(Ok(()), reader.set_cursor(Point::new(1, 0)));
    }
}
