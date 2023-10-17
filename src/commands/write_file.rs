use std::fs::File;
use std::io::Write;

use crate::editor::Editor;

pub struct WriteFileCommand {}

impl WriteFileCommand {
    pub fn run(editor: &mut Editor) {
        let command = editor.command.text.clone();
        let buffer = editor.get_active_buffer_mut();
        let path;

        if command.trim() == "w" {
            if let Some(file_name) = &buffer.file_name {
                path = file_name.as_str();
            } else {
                return;
            }
        } else if command.trim().len() > 2 {
            path = &command[2..];
        } else {
            return;
        }

        if path.starts_with("~/") {
            let mut home_path = home::home_dir().unwrap();
            home_path.push(&path[2..]);
            buffer.file_name = Some(home_path.display().to_string());
        } else {
            buffer.file_name = Some(String::from(path));
        }

        let mut file = File::create(buffer.file_name.as_ref().unwrap()).unwrap();
        file.write_all(buffer.get_content().as_bytes()).unwrap();
    }
}
