use std::fs::File;
use std::io::Write;

use crate::buffer::Buffer;

pub struct WriteFileCommand {}

impl WriteFileCommand {
    pub fn run(buffer: &mut Buffer) {
        if buffer.command_line.text.len() > 2 {
            let file_name = &buffer.command_line.text[2..];
            if file_name.starts_with("~/") {
                let mut file_path = home::home_dir().unwrap();
                file_path.push(&file_name[2..]);
                buffer.file_name = Some(file_path.display().to_string());
            }
        }
        let mut file = File::create(buffer.file_name.as_ref().unwrap()).unwrap();
        file.write_all(buffer.get_content().as_bytes()).unwrap();
    }
}
