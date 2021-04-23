// INCOMPLETE CODE UP AHEAD

use crate::{
    input::Input,
    funcs::count::{char_count, word_count},
    Errors,
};
use std::{
    fmt::Formatter,
    fs::read_to_string, path::{
        Path,
        PathBuf
    },

};
use structopt::StructOpt;

#[derive(Clone)]
pub struct File<'t> {
    pub file_path:          &'t PathBuf,
    pub content:            String,
    pub date_created:       Option<String>,
    pub size:               Option<String>,
    pub total_lines:        usize,
    pub longest_line:       Option<i32>,
    pub character_count:    usize,
    pub encoding:           Option<i32>,
    pub total_words:        usize,
}

impl<'t> File<'t> {
    pub fn from(path: &'t PathBuf) -> Self {
        let file = Path::new(&path);
        if !file.is_file() {
            Errors::throw(Errors::FileNotFound(&file));
        }

        let file_text = read_to_string(file).unwrap();

        Self {
            file_path: path,
            content: file_text,
            date_created: None,
            size: None,
            total_lines: 0,
            longest_line: None,
            character_count: 0,
            encoding: None,
            total_words: 0,
        }
    }
    pub fn parse_from_input(file: &mut File) {
        let input = Input::from_args();
        let content = &file.content;
        if input.character_count {
            file.character_count = char_count(content);
        }
        if input.words_count {
            file.total_words = word_count(content);
        }
    }
}

// impl std::fmt::Display for File {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", out)
//     }
// }
