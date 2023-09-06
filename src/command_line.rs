use std::cmp::{min, max};

pub struct CommandLine {
    pub text: String,
    pub cursor_x: usize,
}

impl CommandLine {
    pub fn insert_key(&mut self, ch: char) {
        self.text.insert(self.cursor_x, ch);
        self.move_right();
    }

    pub fn delete_key(&mut self) {
        if self.cursor_x > 0 {
            self.text.remove(self.cursor_x - 1);
            self.move_left();
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x = max(0, self.cursor_x - 1);
        }
    }

    pub fn move_right(&mut self) {
        self.cursor_x = min(self.text.len(), self.cursor_x + 1);
    }

    pub fn reset(&mut self) {
        self.text = String::new();
        self.cursor_x = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::command_line::CommandLine;

    #[test]
    fn test() {
        let mut command_line = CommandLine {
            text: String::new(),
            cursor_x: 0,
        };

        command_line.reset();

        assert_eq!("", command_line.text);
        assert_eq!(0, command_line.cursor_x);

        command_line.delete_key();
        command_line.move_left();
        command_line.move_right();
        command_line.insert_key('b');

        assert_eq!("b", command_line.text);
        assert_eq!(1, command_line.cursor_x);

        command_line.move_left();

        assert_eq!(0, command_line.cursor_x);

        command_line.insert_key('a');

        assert_eq!("ab", command_line.text);
        assert_eq!(1, command_line.cursor_x);

        command_line.move_right();
        command_line.move_right();
        command_line.insert_key('c');

        assert_eq!("abc", command_line.text);
        assert_eq!(3, command_line.cursor_x);
    }
}
