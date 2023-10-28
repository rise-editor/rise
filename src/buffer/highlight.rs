use crate::{
    buffer::{mode::BufferMode, Buffer},
    core::{rectangle::Rectangle, style::Style, text_reader::TextReader},
    theme::THEME_ONE as T,
};

pub struct Highlight {
    pub name: &'static str,
    pub row: usize,
    pub start: usize,
    pub end: usize,
}

pub const HL_CURRENT_LINE: &str = "CurrentLine";
pub const HL_CURRENT_LINE_TEXT: &str = "CurrentLineText";
pub const HL_FIND_TEXT: &str = "FindText";
pub const HL_SELECTED_TEXT: &str = "SelectedText";

impl Highlight {
    pub fn is_visible_in_area(&self, area: Rectangle<usize>) -> bool {
        !(self.row < area.y
            || area.y + area.height <= self.row
            || self.end < area.x
            || area.x + area.width <= self.start)
    }
}

impl Buffer {
    pub fn set_default_styles(&mut self) {
        self.styles
            .insert(HL_FIND_TEXT, Style::new(T.text_finded_fg, T.text_finded_bg));
        self.styles.insert(
            HL_SELECTED_TEXT,
            Style::new(T.text_selected_fg, T.text_selected_bg),
        );
    }

    pub fn get_dynamic_highlights(&self) -> Vec<Highlight> {
        let mut list: Vec<Highlight> = vec![];

        list.push(Highlight {
            name: HL_CURRENT_LINE,
            row: self.cursor.x,
            start: self.scroll.x,
            end: self.text_area.width as usize,
        });

        if let Some(end) = self.get_current_line_last_char_index() {
            list.push(Highlight {
                name: HL_CURRENT_LINE_TEXT,
                row: self.cursor.y,
                start: 0,
                end,
            });
        }

        if let BufferMode::Visual = self.mode {
            let mut reader = TextReader::new(&self.lines);
            let (from, to) = if self.selection.start < self.cursor {
                (self.selection.start.clone(), self.cursor.clone())
            } else {
                (self.cursor.clone(), self.selection.start.clone())
            };
            let _ = reader.set_cursor(from);

            while reader.get_cursor() <= to {
                let mut highlight = Highlight {
                    name: HL_SELECTED_TEXT,
                    row: reader.get_cursor_y(),
                    start: reader.get_cursor_x(),
                    end: reader.get_cursor_x(),
                };
                while reader.get_cursor() <= to && !reader.is_line_last_x() {
                    let _ = reader.next();
                    highlight.end += 1;
                }
                list.push(highlight);
                let _ = reader.next();
            }
        }

        list
    }

    pub fn clear_highlight(&mut self, name: &str) {
        self.highlights.retain(|h| h.name != name);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        buffer::{Buffer, Highlight},
        core::size::Size,
    };

    fn test1(row: usize, start: usize, end: usize) -> bool {
        let area = Size::new(10, 10).to_rectangle();

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

    #[test]
    fn clear_highlight_test() {
        let mut buffer = Buffer::new(Size::new(10, 10).to_rectangle());

        buffer.highlights.push(Highlight {
            name: "Foo",
            row: 0,
            start: 0,
            end: 1,
        });
        buffer.highlights.push(Highlight {
            name: "Bar",
            row: 0,
            start: 0,
            end: 1,
        });
        buffer.highlights.push(Highlight {
            name: "Baz",
            row: 0,
            start: 0,
            end: 1,
        });

        buffer.clear_highlight("Foo");

        assert_eq!(2, buffer.highlights.len());
    }
}
