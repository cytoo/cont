use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Input {
    /// location to your file
    #[structopt(parse(from_os_str))]
    pub file_path: std::path::PathBuf,
    /// show the lines count
    #[structopt(long = "count", short = "c")]
    pub character_count: bool,

    /// show the count of words
    #[structopt(long = "words", short = "w")]
    pub words_count: bool,
}
