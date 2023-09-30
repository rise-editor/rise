use std::fs;

use crate::{buffer::Buffer, core::Size};

fn get_file_list(directory: &String) -> Vec<String> {
    let paths = fs::read_dir(directory).unwrap();

    let mut files: Vec<String> = vec![];

    files.push(String::from(".."));
    files.push(String::from("."));

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

    files
}

pub fn create_explorer_buffer(base_path: String, area: Size<u16>) -> Buffer {
    let files = get_file_list(&base_path);

    let mut buffer = Buffer::new(area);
    buffer.file_name = Some(base_path);

    buffer.lines.remove(0);
    for file in files {
        buffer.lines.push(file);
    }

    buffer.actions_normal.insert("enter", |editor| {
        let buffer = editor.get_active_buffer_mut();
        let dir = buffer.file_name.as_ref().unwrap();
        let file = buffer.get_current_line().clone();
        let path = format!("{}/{}", dir, file);

        let md = fs::metadata(path.as_str()).unwrap();

        if md.is_file() {
            buffer.command.text = format!("e {}", path);
            buffer.run_command()
        } else if md.is_dir() {
            buffer.file_name = Some(path.clone());

            let files = get_file_list(&path);

            buffer.lines.clear();

            for file in files {
                buffer.lines.push(file);
            }

            buffer.move_cursor(0, 0);
        }
    });

    buffer
}
