use crate::funcs::file::File;
use std::path::{Path, PathBuf};

pub fn word_count(content: &String) -> usize {
    content.split_whitespace().count()
}

pub fn char_count(content: &String) -> usize {
    content.len()
}
