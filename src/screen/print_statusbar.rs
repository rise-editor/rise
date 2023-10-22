use crate::{
    buffer::mode::BufferMode,
    core::style::Style,
    editor::Editor,
    screen::Screen,
    theme::{THEME_ONE as T, WHITE},
};

impl Screen {
    pub fn print_statusbar(&mut self, editor: &Editor) {
        let buffer = editor.get_active_buffer_or_popup();

        self.set_style(
            editor.status_area.y,
            editor.status_area.x,
            editor.status_area.width - 1,
            Style {
                fg: WHITE,
                bg: T.status_line_bg,
                bold: false,
                italic: false,
                underline: false,
            },
        );

        let (mode, fg, bg) = match buffer.mode {
            BufferMode::Normal => (
                " NORMAL ".to_owned(),
                T.status_normal_mode_fg,
                T.status_normal_mode_bg,
            ),
            BufferMode::Insert => (
                " INSERT ".to_owned(),
                T.status_insert_mode_fg,
                T.status_insert_mode_bg,
            ),
            BufferMode::Visual => (
                " VISUAL ".to_owned(),
                T.status_visual_mode_fg,
                T.status_visual_mode_bg,
            ),
            BufferMode::Command => (
                format!(":{}", editor.command.text),
                T.status_command_mode_fg,
                T.status_command_mode_bg,
            ),
        };

        self.print_text(
            editor.status_area.y,
            editor.status_area.x,
            &format!("{}", mode),
            Style {
                fg,
                bg,
                bold: false,
                italic: false,
                underline: false,
            },
        );
    }
}
