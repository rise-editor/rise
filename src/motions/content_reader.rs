use crate::core::point::Point;

#[derive(Clone, Eq, PartialEq)]
pub enum CharType {
    Whitespace,
    Word,
    Special,
}

#[derive(Eq, PartialEq)]
pub enum Direction {
    Backward,
    Forward,
}

pub struct ContentReader<'a> {
    pub lines: Vec<&'a str>,
    pub position: Point<usize>,
}

impl<'a> ContentReader<'a> {
    pub fn move_backward(&mut self) -> bool {
        if self.is_file_first_char() {
            return false;
        }

        if self.is_line_first_char() {
            self.position.y -= 1;
            self.position.x = self.lines.get(self.position.y).unwrap().len();
            if self.position.x > 0 {
                self.position.x -= 1;
            }
        } else {
            self.position.x -= 1;
        }

        true
    }

    pub fn move_forward(&mut self) -> bool {
        if self.is_file_last_char() {
            return false;
        }

        if self.is_line_last_char() {
            self.position.y += 1;
            self.position.x = 0;
        } else {
            self.position.x += 1;
        }

        true
    }

    pub fn is_file_first_char(&self) -> bool {
        self.position.x == 0 && self.position.y == 0
    }

    pub fn is_file_last_char(&self) -> bool {
        if self.position.y != self.lines.len() - 1 {
            return false;
        }

        self.is_line_last_char()
    }

    pub fn is_line_first_char(&self) -> bool {
        self.position.x == 0
    }

    pub fn is_line_last_char(&self) -> bool {
        let line = self.lines.get(self.position.y).unwrap();

        line.len() == 0 || self.position.x == line.len() - 1
    }

    pub fn get_char(&self) -> Option<char> {
        self.lines
            .get(self.position.y)
            .unwrap()
            .chars()
            .nth(self.position.x)
    }

    pub fn get_char_type(&self) -> CharType {
        let ch = self.get_char().unwrap_or(' ');

        if ch.is_whitespace() {
            CharType::Whitespace
        } else if ch.is_alphanumeric() || ch == '_' {
            CharType::Word
        } else {
            CharType::Special
        }
    }

    // TODO: Improve this, add move_while_next_char(...

    pub fn move_while(&mut self, char_type: CharType, direction: Direction) {
        while self.get_char_type() == char_type {
            match direction {
                Direction::Backward => {
                    if self.is_file_first_char() {
                        break;
                    }
                    if char_type != CharType::Whitespace && self.is_line_first_char() {
                        break;
                    }
                    self.move_backward();
                }
                Direction::Forward => {
                    if self.is_file_last_char() {
                        break;
                    }
                    if char_type != CharType::Whitespace && self.is_line_last_char() {
                        break;
                    }
                    self.move_forward();
                }
            };
        }
    }
}
