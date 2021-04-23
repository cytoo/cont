// INCOMPLETE CODE UP AHEAD
use std::{fs::read_to_string, path::Path};
use syntect::{
    easy::HighlightLines,
    parsing::SyntaxSet,
    highlighting::{
        ThemeSet, Style
    },
    util::{
        as_24_bit_terminal_escaped,
        LinesWithEndings
    }
};
use structopt::StructOpt;

// use funcs::size::get_file_size;
use errors::*;
use input::Input;

mod errors;
mod funcs;
mod input;
use crate::errors::Errors::FileNotFound;
use funcs::file::File;
use std::borrow::{Borrow, BorrowMut};
use colored::Colorize;
use std::io::Lines;

fn main() {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let inp = Input::from_args();
    let mut f = File::from(&inp.file_path);
    File::parse_from_input(&mut f);
    let fe = Path::new(f.file_path);
    let fname = fe.file_name().unwrap().to_str().unwrap().yellow();
    let mut ext = "";
    if fname.contains(".") {
        let vec_split = fname.split(".");
        let mut vec: Vec<&str> = vec_split.collect();
        ext = vec.pop().unwrap();
    }
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let mut line_cnt = 1;
    let file_string = read_to_string(fe.to_path_buf()).unwrap();
    for line in file_string.lines() {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped_text = as_24_bit_terminal_escaped(&ranges[..], false);
        println!("{}{} {}", line_cnt.to_string().as_str().bright_yellow(), "|".yellow(), escaped_text);
        line_cnt += 1;
    }
    println!(
        "\n{} {} {} {} {}\n",
        fname,
        f.total_words.to_string().as_str().bright_cyan(),
        "words".bright_green(),
        f.character_count.to_string().as_str().bright_cyan(),
        "chars".bright_green()
    );
}
