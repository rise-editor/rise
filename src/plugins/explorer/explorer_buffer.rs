use std::{fs, path::PathBuf};

use crate::buffer::Buffer;

fn get_file_list(directory: &String) -> Vec<String> {
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

pub fn initialize_explorer_buffer(buffer: &mut Buffer, base_path: String) {
    let files = get_file_list(&base_path);

    buffer.file_name = Some(base_path);

    buffer.lines.clear();
    for file in files {
        buffer.lines.push(file);
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

        let md = fs::metadata(&path).unwrap();

        if md.is_file() {
            buffer.command.text = format!("e {}", path);
            buffer.run_command();
        } else if md.is_dir() {
            buffer.file_name = Some(path.clone());

            let files = get_file_list(&path);

            buffer.lines.clear();

            for file in files {
                buffer.lines.push(file);
            }

            buffer.move_cursor(0, 0);
            buffer.set_size(buffer.area.clone());
        }
    });

    buffer.set_size(buffer.area.clone());
}
