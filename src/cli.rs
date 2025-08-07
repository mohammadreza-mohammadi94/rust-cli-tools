use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "fastgrep",
    version = "0.1.0",
    about = "A high-performance text search tool",
    long_about = "A fast, parallel text search tool similar to ripgrep"
)]
pub struct Args {
    /// The pattern to search for
    #[arg(value_name = "PATTERN")]
    pub pattern: String,

    /// Files or directories to search
    #[arg(value_name = "PATH")]
    pub paths: Vec<PathBuf>,

    /// Perform case insensitive matching
    #[arg(short = 'i', long)]
    pub ignore_case: bool,

    /// Only show matches surrounded by word boundaries
    #[arg(short = 'w', long)]
    pub word_regexp: bool,

    /// Treat the pattern as a literal string
    #[arg(short = 'F', long)]
    pub fixed_strings: bool,

    /// Show NUM lines before each match
    #[arg(short = 'B', long, value_name = "NUM", default_value = "0")]
    pub before_context: usize,

    /// Show NUM lines after each match
    #[arg(short = 'A', long, value_name = "NUM", default_value = "0")]
    pub after_context: usize,

    /// Show NUM lines before and after each match
    #[arg(short = 'C', long, value_name = "NUM")]
    pub context: Option<usize>,

    /// Limit the number of matching lines per file
    #[arg(short = 'm', long, value_name = "NUM")]
    pub max_count: Option<usize>,

    /// Only print the names of files that contain matches
    #[arg(short = 'l', long)]
    pub files_with_matches: bool,

    /// Only print the names of files that don't contain matches
    #[arg(short = 'L', long)]
    pub files_without_matches: bool,

    /// Suppress file names from output
    #[arg(short = 'h', long)]
    pub no_filename: bool,

    /// Always print file names
    #[arg(short = 'H', long)]
    pub with_filename: bool,

    /// Print line numbers
    #[arg(short = 'n', long)]
    pub line_number: bool,

    /// Search hidden files and directories
    #[arg(long)]
    pub hidden: bool,

    /// Don't respect .gitignore files
    #[arg(long)]
    pub no_ignore: bool,

    /// Recursively search subdirectories
    #[arg(short = 'r', long, default_value = "true")]
    pub recursive: bool,

    /// Maximum search depth
    #[arg(long, value_name = "NUM")]
    pub max_depth: Option<usize>,

    /// File types to include (e.g., rust, python)
    #[arg(short = 't', long, value_name = "TYPE")]
    pub type_add: Vec<String>,

    /// File types to exclude
    #[arg(short = 'T', long, value_name = "TYPE")]
    pub type_not: Vec<String>,

    /// Enable colored output
    #[arg(long, default_value = "auto", value_name = "WHEN")]
    pub color: String,
}

impl Args {
    pub fn should_show_filename(&self, multiple_files: bool) -> bool {
        if self.no_filename {
            false
        } else if self.with_filename {
            true
        } else {
            multiple_files
        }
    }

    pub fn get_context(&self) -> (usize, usize) {
        if let Some(context) = self.context {
            (context, context)
        } else {
            (self.before_context, self.after_context)
        }
    }
}
