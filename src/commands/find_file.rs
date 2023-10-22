use std::process::Command;

use crate::{
    buffer::Buffer,
    core::{rectangle::Rectangle, style::Style},
    editor::Editor,
    theme::SILVER,
};

pub struct FindFileCommand {}

impl FindFileCommand {
    pub fn run(editor: &mut Editor) {
        let buffer = editor.get_active_buffer_mut();
        let mut files_popup = Buffer::new(Rectangle {
            x: 10,
            y: 10,
            width: buffer.area.width - 20,
            height: buffer.area.height - 21,
        });
        files_popup.options.show_border = true;
        files_popup.options.show_info_column = false;
        files_popup.set_size(files_popup.area.clone()); // TODO: Move
        files_popup.styles.insert(
            "CurrentLineText",
            Style {
                fg: (88, 129, 87),
                bg: SILVER,
                bold: false,
                italic: false,
                underline: false,
            },
        );

        let mut textbox_popup = Buffer::new(Rectangle {
            x: files_popup.area.x,
            y: files_popup.area.y + files_popup.area.height + 2,
            width: files_popup.area.width,
            height: 1,
        });
        textbox_popup.options.show_border = true;
        textbox_popup.options.show_info_column = false;
        textbox_popup.set_size(textbox_popup.area.clone()); // TODO: Move

        textbox_popup.actions_insert.insert("<c-p>", |editor| {
            let buffer = editor.get_active_buffer_mut();
            let popup_index = buffer.popups.len() - 2;
            let popup = buffer.popups.get_mut(popup_index).unwrap();
            popup.move_up();
        });

        textbox_popup.actions_insert.insert("<c-n>", |editor| {
            let buffer = editor.get_active_buffer_mut();
            let popup_index = buffer.popups.len() - 2;
            let popup = buffer.popups.get_mut(popup_index).unwrap();
            popup.move_down();
        });
        textbox_popup.enter_insert_mode();

        textbox_popup.actions_insert.insert("tab", |editor| {
            let popup = editor.get_active_buffer_or_popup_mut();
            let text = popup.lines.get(0).unwrap();
            let output = Command::new("rg")
                .arg("-g")
                .arg("*".to_string() + text + "*")
                .arg("--files")
                .output()
                .unwrap();
            let str = String::from_utf8_lossy(&output.stdout);
            let buffer = editor.get_active_buffer_mut();
            let popup_index = buffer.popups.len() - 2;
            let popup = buffer.popups.get_mut(popup_index).unwrap();
            popup.set_content(String::from(str));
        });
        textbox_popup.actions_insert.insert("enter", |editor| {
            let buffer = editor.get_active_buffer();
            let popup = buffer.popups.get(buffer.popups.len() - 2).unwrap();
            let filename = popup.get_line(popup.cursor.y);
            editor.command.text = format!("e {}", filename).to_string();

            let buffer = editor.get_active_buffer_mut();
            buffer.popups.pop();
            buffer.popups.pop();
            buffer.active_popup = None;

            editor.run_command();
        });

        textbox_popup.actions_normal.insert("esc", |editor| {
            let buffer = editor.get_active_buffer_mut();
            buffer.popups.pop();
            buffer.popups.pop();
            buffer.active_popup = None;
        });

        buffer.popups.push(files_popup);
        buffer.popups.push(textbox_popup);
        buffer.active_popup = Some(buffer.popups.len() - 1);
    }
}
