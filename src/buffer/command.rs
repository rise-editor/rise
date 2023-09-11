use crate::buffer::Buffer;
use crate::commands::{read_file::ReadFileCommand, write_file::WriteFileCommand};

impl Buffer {
    pub fn run_command(&mut self) {
        if self.command.text.starts_with("w ") {
            WriteFileCommand::run(self);
        } else if self.command.text.starts_with("e ") {
            ReadFileCommand::run(self);
        }
        self.command.reset();
        self.enter_normal_mode();
    }
}
