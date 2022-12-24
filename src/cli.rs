use std::path::PathBuf;

/// Generate Anki cards from Markdown notes
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Markdown files
    #[arg()]
    pub file: Vec<String>,

    /// Deck name
    #[arg(short, long, default_value_t = String::from("Marki"))]
    pub deck: String,

    /// Output file
    #[arg(short, long, default_value_os_t = PathBuf::from("marki.apkg"))]
    pub output: PathBuf,

    #[arg(short, long)]
    pub verbose: bool,
}
