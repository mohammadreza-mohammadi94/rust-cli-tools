use anyhow::{Context, Result};
use rayon::prelude::*;
use regex::{Regex, RegexBuilder};
use std::path::{Path, PathBuf};
use memmap2::Mmap;
use std::fs::File;

use crate::cli::Args;
use crate::output::OutputFormatter;
use crate::walker::FileWalker;

pub struct SearchEngine {
    regex: Regex,
    before_context: usize,
    after_context: usize,
    max_count: Option<usize>,
}

impl SearchEngine {
    pub fn new(
        pattern: &str,
        ignore_case: bool,
        word_regexp: bool,
        fixed_strings: bool,
    ) -> Result<Self> {
        let pattern = if fixed_strings {
            regex::escape(pattern)
        } else if word_regexp {
            format!(r"\b{}\b", pattern)
        } else {
            pattern.to_string()
        };

        let regex = RegexBuilder::new(&pattern)
            .case_insensitive(ignore_case)
            .multi_line(true)
            .build()
            .context("Failed to compile regex pattern")?;

        Ok(SearchEngine {
            regex,
            before_context: 0,
            after_context: 0,
            max_count: None,
        })
    }

    pub fn set_context(&mut self, before: usize, after: usize) {
        self.before_context = before;
        self.after_context = after;
    }

    pub fn set_max_count(&mut self, max_count: Option<usize>) {
        self.max_count = max_count;
    }

    pub fn search_paths(&self, paths: &[PathBuf], args: &Args) -> Result<()> {
        let walker = FileWalker::new(args)?;
        let files: Vec<PathBuf> = walker.walk_paths(paths)?;
        
        let multiple_files = files.len() > 1;
        let formatter = OutputFormatter::new(args, multiple_files);

        // Use parallel processing for multiple files
        if files.len() > 1 {
            files.par_iter().for_each(|file| {
                if let Err(e) = self.search_file(file, &formatter, args) {
                    eprintln!("Error searching {}: {}", file.display(), e);
                }
            });
        } else {
            for file in &files {
                self.search_file(file, &formatter, args)?;
            }
        }

        Ok(())
    }

    fn search_file(&self, path: &Path, formatter: &OutputFormatter, args: &Args) -> Result<()> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let content = std::str::from_utf8(&mmap)
            .context("File contains invalid UTF-8")?;

        let mut matches = Vec::new();
        let mut match_count = 0;

        for (line_num, line) in content.lines().enumerate() {
            if let Some(max) = self.max_count {
                if match_count >= max {
                    break;
                }
            }

            if self.regex.is_match(line) {
                matches.push(MatchResult {
                    line_number: line_num + 1,
                    line_content: line.to_string(),
                    byte_offset: 0, // Could be calculated if needed
                });
                match_count += 1;
            }
        }

        if args.files_with_matches && !matches.is_empty() {
            formatter.print_filename_only(path);
            return Ok(());
        }

        if args.files_without_matches && matches.is_empty() {
            formatter.print_filename_only(path);
            return Ok(());
        }

        if !matches.is_empty() {
            let content_lines: Vec<&str> = content.lines().collect();
            self.print_matches_with_context(
                path,
                &matches,
                &content_lines,
                formatter,
                args,
            )?;
        }

        Ok(())
    }

    fn print_matches_with_context(
        &self,
        path: &Path,
        matches: &[MatchResult],
        content_lines: &[&str],
        formatter: &OutputFormatter,
        args: &Args,
    ) -> Result<()> {
        let mut printed_lines = std::collections::HashSet::new();

        for match_result in matches {
            let line_idx = match_result.line_number - 1;
            
            let start_ctx = line_idx.saturating_sub(self.before_context);
            let end_ctx = std::cmp::min(
                line_idx + self.after_context + 1,
                content_lines.len(),
            );

            // Print context before
            for i in start_ctx..line_idx {
                if !printed_lines.contains(&i) {
                    formatter.print_context_line(
                        path,
                        i + 1,
                        content_lines[i],
                        false,
                        args,
                    )?;
                    printed_lines.insert(i);
                }
            }

            // Print the match
            if !printed_lines.contains(&line_idx) {
                formatter.print_match_line(
                    path,
                    match_result.line_number,
                    &match_result.line_content,
                    &self.regex,
                    args,
                )?;
                printed_lines.insert(line_idx);
            }

            // Print context after
            for i in (line_idx + 1)..end_ctx {
                if !printed_lines.contains(&i) {
                    formatter.print_context_line(
                        path,
                        i + 1,
                        content_lines[i],
                        false,
                        args,
                    )?;
                    printed_lines.insert(i);
                }
            }

            // Print separator if there are more matches
            if self.before_context > 0 || self.after_context > 0 {
                if matches.len() > 1 && match_result != matches.last().unwrap() {
                    formatter.print_separator();
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct MatchResult {
    line_number: usize,
    line_content: String,
    byte_offset: usize,
}
