use std::fs::{self, File};
use std::io::Read;

use crate::{commands::explorer::ExplorerCommand, editor::Editor};

pub struct ReadFileCommand {}

impl ReadFileCommand {
    pub fn run(editor: &mut Editor) {
        let command = editor.input.text.clone();
        let buffer = editor.get_active_tab_mut().create_new_buffer();
        let file_name = &command.trim()[2..];
        if file_name.starts_with("~/") {
            let mut home_path = home::home_dir().unwrap();
            home_path.push(&file_name[2..]);
            buffer.file_name = Some(home_path.display().to_string());
        } else {
            buffer.file_name = Some(String::from(file_name));
        }
        let mut content = String::new();
        let file_path = buffer.file_name.as_ref().unwrap();
        if fs::metadata(&file_path).unwrap().is_dir() {
            let path = file_path.clone();
            ExplorerCommand::run(editor, &path);
        } else {
            let mut file = File::open(file_path).unwrap();
            file.read_to_string(&mut content).unwrap();
            buffer.set_content(content);
        }
    }
}
