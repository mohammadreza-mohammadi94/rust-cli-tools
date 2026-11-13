use anyhow::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

use crate::cli::Args;

pub struct FileWalker {
    builder: WalkBuilder,
}

impl FileWalker {
    pub fn new(args: &Args) -> Result<Self> {
        let mut builder = WalkBuilder::new(".");
        
        builder
            .hidden(!args.hidden)
            .ignore(!args.no_ignore)
            .git_ignore(!args.no_ignore)
            .git_exclude(!args.no_ignore);

        if let Some(depth) = args.max_depth {
            builder.max_depth(Some(depth));
        }

        // Add file type filters
        if !args.type_add.is_empty() || !args.type_not.is_empty() {
            let mut types = ignore::types::TypesBuilder::new();
            types.add_defaults();
            
            for type_name in &args.type_add {
                types.select(type_name);
            }
            
            for type_name in &args.type_not {
                types.negate(type_name);
            }
            
            if let Ok(types) = types.build() {
                builder.types(types);
            }
        }

        Ok(FileWalker { builder })
    }

    pub fn walk_paths(&self, paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        for path in paths {
            if path.is_file() {
                files.push(path.clone());
            } else if path.is_dir() {
                let mut walker = self.builder.clone();
                walker.add(path);
                
                let walk = walker.build();
                for entry in walk {
                    match entry {
                        Ok(entry) => {
                            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                                if self.is_text_file(entry.path()) {
                                    files.push(entry.path().to_path_buf());
                                }
                            }
                        }
                        Err(_) => continue,
                    }
                }
            }
        }

        Ok(files)
    }

    fn is_text_file(&self, path: &Path) -> bool {
        // Simple heuristic: check file extension and avoid binary files
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "exe" | "dll" | "so" | "dylib" | "a" | "o" | "bin" | 
                "jpg" | "jpeg" | "png" | "gif" | "bmp" | "ico" | "svg" |
                "mp3" | "mp4" | "avi" | "mov" | "zip" | "tar" | "gz" |
                "pdf" | "doc" | "docx" | "xls" | "xlsx" => false,
                _ => true,
            }
        } else {
            // Files without extension - try to detect if they're text
            true
        }
    }
}