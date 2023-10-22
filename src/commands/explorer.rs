use std::{fs, path::PathBuf};

use crate::{
    buffer::{highlight::Highlight, Buffer},
    core::style::Style,
    editor::Editor,
    theme::{GREEN, SILVER},
};

pub struct ExplorerCommand {}

impl ExplorerCommand {
    pub fn run(editor: &mut Editor, path: &str) {
        let mut buffer = editor.get_active_tab_mut().create_new_buffer();
        initialize_explorer_buffer(&mut buffer, path);
    }
}

const FOLDER_STYLE: &str = "ExplorerFolder";

fn initialize_explorer_buffer(buffer: &mut Buffer, base_path: &str) {
    buffer.styles.insert(
        FOLDER_STYLE,
        Style {
            fg: GREEN,
            bg: SILVER,
            bold: false,
            italic: false,
            underline: false,
        },
    );

    let files = get_file_list(base_path);

    buffer.file_name = Some(base_path.to_owned());
    buffer.lines.clear();
    buffer.highlights.clear();

    for (i, file) in files.iter().enumerate() {
        let mut file_path = PathBuf::from(base_path);
        file_path.push(file);

        if file != "." && file != ".." && fs::metadata(file_path).unwrap().is_dir() {
            buffer.highlights.push(Highlight {
                name: FOLDER_STYLE,
                row: i,
                start: 0,
                end: file.len() - 1,
            });
        }

        buffer.lines.push(file.to_owned());
    }

    buffer.actions_normal.insert("enter", |editor| {
        let buffer = editor.get_active_buffer_mut();
        let dir = buffer.file_name.as_ref().unwrap();
        let file = buffer.get_current_line().clone();
        let mut path_buf = PathBuf::from(dir);

        if file == String::from(".") {
        } else if file == String::from("..") {
            path_buf.pop();
        } else {
            path_buf.push(file);
        }

        let mut path = path_buf.into_os_string().into_string().unwrap();

        if path == "" {
            path = String::from(".");
        }

        editor.command.text = format!("e {}", path);
        editor.run_command();
    });

    buffer.set_size(buffer.area.clone());
}

fn get_file_list(directory: &str) -> Vec<String> {
    let paths = fs::read_dir(directory).unwrap();

    let mut files: Vec<String> = vec![];

    for entry in paths {
        let path = entry
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        files.push(path);
    }

    files.sort();

    files.insert(0, String::from(".."));
    files.insert(1, String::from("."));

    files
}
