use std::fs::File;
use std::io::Read;

use crate::buffer::Buffer;

pub struct ReadFileCommand {}

impl ReadFileCommand {
    pub fn run(buffer: &mut Buffer) {
        if buffer.command.text.len() > 2 {
            let file_name = &buffer.command.text[2..];
            if file_name.starts_with("~/") {
                let mut home_path = home::home_dir().unwrap();
                home_path.push(&file_name[2..]);
                buffer.file_name = Some(home_path.display().to_string());
            } else {
                buffer.file_name = Some(String::from(file_name));
            }
            let mut content = String::new();
            let mut file = File::open(buffer.file_name.as_ref().unwrap()).unwrap();
            file.read_to_string(&mut content).unwrap();
            buffer.set_content(content);
        }
    }
}
