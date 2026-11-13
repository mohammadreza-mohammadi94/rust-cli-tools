use anyhow::Result;
use colored::Colorize;
use regex::Regex;
use std::path::Path;
use std::sync::Mutex;

use crate::cli::Args;

pub struct OutputFormatter {
    show_filename: bool,
    show_line_numbers: bool,
    use_colors: bool,
    output_lock: Mutex<()>,
}

impl OutputFormatter {
    pub fn new(args: &Args, multiple_files: bool) -> Self {
        let use_colors = match args.color.as_str() {
            "always" => true,
            "never" => false,
            "auto" | _ => atty::is(atty::Stream::Stdout),
        };

        OutputFormatter {
            show_filename: args.should_show_filename(multiple_files),
            show_line_numbers: args.line_number,
            use_colors,
            output_lock: Mutex::new(()),
        }
    }

    pub fn print_filename_only(&self, path: &Path) {
        let _lock = self.output_lock.lock().unwrap();
        if self.use_colors {
            println!("{}", path.display().to_string().purple());
        } else {
            println!("{}", path.display());
        }
    }

    pub fn print_match_line(
        &self,
        path: &Path,
        line_number: usize,
        line: &str,
        regex: &Regex,
        _args: &Args,
    ) -> Result<()> {
        let _lock = self.output_lock.lock().unwrap();
        
        let mut output = String::new();

        if self.show_filename {
            let filename = if self.use_colors {
                path.display().to_string().purple().to_string()
            } else {
                path.display().to_string()
            };
            output.push_str(&filename);
            output.push(':');
        }

        if self.show_line_numbers {
            let line_num = if self.use_colors {
                line_number.to_string().green().to_string()
            } else {
                line_number.to_string()
            };
            output.push_str(&line_num);
            output.push(':');
        }

        let highlighted_line = if self.use_colors {
            self.highlight_matches(line, regex)
        } else {
            line.to_string()
        };

        output.push_str(&highlighted_line);
        println!("{}", output);

        Ok(())
    }

    pub fn print_context_line(
        &self,
        path: &Path,
        line_number: usize,
        line: &str,
        _is_after: bool,
        _args: &Args,
    ) -> Result<()> {
        let _lock = self.output_lock.lock().unwrap();
        
        let mut output = String::new();

        if self.show_filename {
            let filename = if self.use_colors {
                path.display().to_string().purple().to_string()
            } else {
                path.display().to_string()
            };
            output.push_str(&filename);
            output.push('-');
        }

        if self.show_line_numbers {
            let line_num = if self.use_colors {
                line_number.to_string().cyan().to_string()
            } else {
                line_number.to_string()
            };
            output.push_str(&line_num);
            output.push('-');
        }

        output.push_str(line);
        println!("{}", output);

        Ok(())
    }

    pub fn print_separator(&self) {
        let _lock = self.output_lock.lock().unwrap();
        println!("--");
    }

    fn highlight_matches(&self, line: &str, regex: &Regex) -> String {
        let mut result = String::new();
        let mut last_match = 0;

        for mat in regex.find_iter(line) {
            result.push_str(&line[last_match..mat.start()]);
            result.push_str(&line[mat.start()..mat.end()].red().bold().to_string());
            last_match = mat.end();
        }
        result.push_str(&line[last_match..]);

        result
    }
}