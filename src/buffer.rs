pub mod find;
pub mod mode;
pub mod movement;
pub mod operations;
pub mod paint_helper;
pub mod text;

use crate::command_line::CommandLine;
use crate::commands::{read_file::ReadFileCommand, write_file::WriteFileCommand};
use crate::core::{Point, Size};

use self::mode::BufferMode;

pub struct Buffer {
    pub file_name: Option<String>,
    pub mode: BufferMode,
    pub visible_area: Size<u16>,
    pub scroll: Point<usize>,
    pub cursor: Point<usize>,
    pub lines: Vec<String>,
    pub command_line: CommandLine,
}

impl Buffer {
    pub fn run_command(&mut self) {
        if self.command_line.text.starts_with("w ") {
            WriteFileCommand::run(self);
        } else if self.command_line.text.starts_with("e ") {
            ReadFileCommand::run(self);
        }
        self.command_line.reset();
        self.enter_normal_mode();
    }
}
