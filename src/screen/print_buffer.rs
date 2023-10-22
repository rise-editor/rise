use std::cmp::{max, min};

use crate::{
    buffer::{highlight::Highlight, Buffer},
    core::{Rectangle, Style},
    screen::Screen,
    theme::THEME_ONE as T,
};

impl Screen {
    fn print_buffer_info_column(&mut self, buffer: &Buffer) {
        if buffer.options.show_border {
            for column in 0..buffer.area.width {
                let cell_top = self
                    .get_cell_mut(buffer.area.y - 1, buffer.area.x + column)
                    .unwrap();
                cell_top.char = '-';
                cell_top.style.fg = T.border_color_fg;
                cell_top.style.bg = T.border_color_bg;

                let cell_bottom = self
                    .get_cell_mut(buffer.area.y + buffer.area.height, buffer.area.x + column)
                    .unwrap();
                cell_bottom.char = '-';
                cell_bottom.style.fg = T.border_color_fg;
                cell_bottom.style.bg = T.border_color_bg;
            }
            for row in 0..buffer.area.height {
                let cell_left = self
                    .get_cell_mut(buffer.area.y + row, buffer.area.x - 1)
                    .unwrap();
                cell_left.char = '|';
                cell_left.style.fg = T.border_color_fg;
                cell_left.style.bg = T.border_color_bg;

                let cell_right = self
                    .get_cell_mut(buffer.area.y + row, buffer.area.x + buffer.area.width)
                    .unwrap();
                cell_right.char = '|';
                cell_right.style.fg = T.border_color_fg;
                cell_right.style.bg = T.border_color_bg;
            }
        }
    }

    fn print_highlight(&mut self, buffer: &Buffer, highlight: &Highlight) {
        if !highlight.is_visible_in_area(Rectangle {
            x: buffer.scroll.x,
            y: buffer.scroll.y,
            width: buffer.text_area.width as usize,
            height: buffer.text_area.height as usize,
        }) {
            return;
        }

        if let Some(style) = buffer.styles.get(highlight.name) {
            let row = (highlight.row - buffer.scroll.y) as u16;
            let start = (max(buffer.scroll.x, highlight.start) - buffer.scroll.x) as u16;
            let end = (min(
                buffer.scroll.x + buffer.text_area.width as usize - 1,
                highlight.end,
            ) - buffer.scroll.x) as u16;

            self.set_style(
                buffer.text_area.y + row,
                buffer.text_area.x + start,
                buffer.text_area.x + end,
                style.clone(),
            );
        }
    }

    fn print_highlights(&mut self, buffer: &Buffer) {
        for highlight in buffer.highlights.iter() {
            self.print_highlight(buffer, &highlight);
        }

        for highlight in buffer.get_dynamic_highlights().iter() {
            self.print_highlight(buffer, &highlight);
        }
    }

    pub fn print_buffer(&mut self, buffer: &Buffer) {
        self.clear_area(buffer.area.clone());

        self.print_buffer_info_column(buffer);

        for y in 0..buffer.area.height {
            let row_index = buffer.scroll.y + y as usize;
            match buffer.get_line_visible_text(row_index) {
                Some(text) => {
                    // TODO: Move this and print in separate functions
                    if buffer.options.show_info_column {
                        self.print_text(
                            buffer.area.y + y,
                            buffer.area.x,
                            &format!(
                                " {:>1$} ",
                                row_index + 1,
                                buffer.info_area.width as usize - 2
                            ),
                            Style {
                                fg: T.info_column_fg,
                                bg: T.info_column_bg,
                                bold: false,
                                italic: false,
                                underline: false,
                            },
                        );
                    }
                    self.print_text(
                        buffer.area.y + y,
                        buffer.text_area.x,
                        &text,
                        Style {
                            fg: T.text_fg,
                            bg: T.text_bg,
                            bold: false,
                            italic: false,
                            underline: false,
                        },
                    );
                }
                None => {
                    if buffer.options.show_info_column {
                        self.print_text(
                            buffer.area.y + y,
                            buffer.area.x,
                            "~",
                            Style {
                                fg: T.info_column_fg,
                                bg: T.bg,
                                bold: false,
                                italic: false,
                                underline: false,
                            },
                        );
                    }
                }
            }
        }

        self.print_highlights(buffer);
    }
}
