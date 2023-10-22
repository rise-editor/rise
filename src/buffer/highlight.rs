use crate::{buffer::Buffer, core::Rectangle};

pub struct Highlight {
    pub name: &'static str,
    pub row: usize,
    pub start: usize,
    pub end: usize,
}

impl Highlight {
    pub fn is_visible_in_area(&self, area: Rectangle<usize>) -> bool {
        !(self.row < area.y
            || area.y + area.height <= self.row
            || self.end < area.x
            || area.x + area.width <= self.start)
    }
}

impl Buffer {
    pub fn get_dynamic_highlights(&self) -> Vec<Highlight> {
        let mut list: Vec<Highlight> = vec![];

        list.push(Highlight {
            name: "CurrentLine",
            row: self.cursor.x,
            start: self.scroll.x,
            end: self.text_area.width as usize,
        });

        if let Some(end) = self.get_current_line_last_char_index() {
            list.push(Highlight {
                name: "CurrentLineText",
                row: self.cursor.y,
                start: 0,
                end,
            });
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use crate::{buffer::Highlight, core::Rectangle};

    fn test1(row: usize, start: usize, end: usize) -> bool {
        let area = Rectangle {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };

        let highlight = Highlight {
            name: "Test",
            row,
            start,
            end,
        };

        highlight.is_visible_in_area(area)
    }

    #[test]
    fn is_visible_in_area_test() {
        assert_eq!(true, test1(0, 0, 8));
        assert_eq!(true, test1(0, 1, 9));
        assert_eq!(true, test1(0, 0, 9));
        assert_eq!(true, test1(0, 0, 10));
        assert_eq!(false, test1(0, 10, 10));
        assert_eq!(false, test1(10, 0, 1));
    }
}
