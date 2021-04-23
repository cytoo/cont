use colored::Colorize;
use std::path::Path;

#[derive(Debug)]
pub enum Errors<'t> {
    FileNotFound(&'t Path),
}

impl<'t> Errors<'t> {
    pub fn throw(err: Errors) {
        match err {
            Errors::FileNotFound(path) => {
                println!("{} `{}`", "no such file or directory".red(), path.display())
            }
        }
    }
}
