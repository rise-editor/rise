use crate::{
    buffer::{
        highlight::{Highlight, HL_FIND_TEXT},
        Buffer,
    },
    core::text_position::TextPosition,
};

impl Buffer {
    pub fn find(&mut self, text: &str) {
        self.clear_highlight(HL_FIND_TEXT);
        self.finds.clear();

        if text.len() == 0 {
            return;
        }

        for row in 0..self.get_line_count() {
            let line = self.get_line(row).unwrap().clone();
            if text.len() <= line.len() {
                let column_end = line.len() - text.len() + 1;
                for column in 0..column_end {
                    if &line[column..(column + text.len())] == text {
                        self.finds.push(TextPosition {
                            row,
                            start: column,
                            end: column + text.len() - 1,
                        });
                        self.highlights.push(Highlight {
                            name: HL_FIND_TEXT,
                            row,
                            start: column,
                            end: column + text.len() - 1,
                        });
                    }
                }
            }
        }
    }

    pub fn clear_finds(&mut self) {
        self.finds.clear();
        self.clear_highlight(HL_FIND_TEXT);
    }

    pub fn move_to_next_find(&mut self) {
        if self.finds.len() == 0 {
            return;
        }

        let mut pos: Option<TextPosition> = None;
        for find in self.finds.iter() {
            if find.row < self.cursor.y {
                continue;
            } else if find.row == self.cursor.y && find.start <= self.cursor.x {
                continue;
            } else {
                pos = Some(find.clone());
                break;
            }
        }
        if let Some(p) = pos {
            self.move_cursor(p.row, p.start);
        } else {
            let p = self.finds.get(0).unwrap();
            self.move_cursor(p.row, p.start);
        }
    }

    pub fn move_to_previous_find(&mut self) {
        if self.finds.len() == 0 {
            return;
        }

        let mut pos: Option<TextPosition> = None;
        for find in self.finds.iter().rev() {
            if self.cursor.y < find.row {
                continue;
            } else if find.row == self.cursor.y && self.cursor.x <= find.start {
                continue;
            } else {
                pos = Some(find.clone());
                break;
            }
        }
        if let Some(p) = pos {
            self.move_cursor(p.row, p.start);
        } else {
            let p = self.finds.last().unwrap();
            self.move_cursor(p.row, p.start);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{buffer::Buffer, core::size::Size};

    #[test]
    fn test() {
        let mut buffer = Buffer::new(Size::new(10, 10).to_rectangle());

        buffer.lines.clear();
        buffer.lines.push(String::from("123 567 90"));
        buffer.lines.push(String::from("qwer yuiio"));
        buffer.lines.push(String::from(""));
        buffer.lines.push(String::from("1 3  56"));
        buffer.lines.push(String::from(""));

        buffer.find("56");

        assert_eq!(2, buffer.finds.len());
        assert_eq!(0, buffer.finds.get(0).unwrap().row);
        assert_eq!(4, buffer.finds.get(0).unwrap().start);
        assert_eq!(5, buffer.finds.get(0).unwrap().end);
        assert_eq!(3, buffer.finds.get(1).unwrap().row);
        assert_eq!(5, buffer.finds.get(1).unwrap().start);
        assert_eq!(6, buffer.finds.get(1).unwrap().end);
    }
}
