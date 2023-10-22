use crate::{
    core::style::Style,
    editor::Editor,
    screen::Screen,
    theme::{THEME_ONE as T, WHITE},
};

impl Screen {
    pub fn print_tabs(&mut self, editor: &Editor) {
        self.set_style(
            editor.tabs_area.y,
            editor.tabs_area.x,
            editor.tabs_area.x + editor.tabs_area.width - 1,
            Style {
                fg: WHITE,
                bg: T.status_line_bg,
                bold: false,
                italic: false,
                underline: false,
            },
        );

        let row = editor.tabs_area.y;
        let mut column: u16 = 0;

        for tab_index in 0..editor.tabs.len() {
            let tab = editor.tabs.get(tab_index).unwrap();
            let buffer = tab.get_active_buffer();

            let (fg, bg) = if editor.active_tab == tab_index {
                (T.tab_selected_fg, T.tab_selected_bg)
            } else {
                (T.tab_fg, T.tab_bg)
            };

            let text = match &buffer.file_name {
                Some(name) => &name,
                None => "[No Name]",
            };

            self.print_text(
                row,
                column,
                format!(" {} ", text).as_str(),
                Style {
                    fg,
                    bg,
                    bold: false,
                    italic: false,
                    underline: false,
                },
            );

            column += text.len() as u16 + 2;
        }
    }
}
