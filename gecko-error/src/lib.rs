#[derive(Debug)]
pub struct LineInfo {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl LineInfo {
    pub fn new(line: usize, start: usize, end: usize) -> LineInfo {
        LineInfo {
            line,
            start,
            end,
        }
    }
}

#[derive(Debug)]
pub enum Note {
    Note(String),
    Expected(String),
}

#[derive(Debug)]
pub struct Error {
    pub line_info: LineInfo,
    pub message: String,
    pub notes: Vec<Note>,
}

impl Error {
    pub fn new(line_info: LineInfo, message: String) -> Error {
        Error {
            line_info,
            message,
            notes: vec![],
        }
    }

    pub fn new_with_notes(line_info: LineInfo, message: String, notes: Vec<Note>) -> Error {
        Error {
            line_info,
            message,
            notes,
        }
    }

    #[allow(dead_code)]
    fn display(&self) {
        todo!()
    }
}
