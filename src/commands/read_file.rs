use std::fs::File;
use std::io::Read;

use crate::editor::Editor;

pub struct ReadFileCommand {}

impl ReadFileCommand {
    pub fn run(editor: &mut Editor) {
        let command = editor.command.text.clone();
        let buffer = editor.get_active_buffer_mut();
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
        let mut file = File::open(file_path).unwrap();
        file.read_to_string(&mut content).unwrap();
        buffer.set_content(content);
    }
}
