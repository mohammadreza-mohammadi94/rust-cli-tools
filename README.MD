# FastGrep

A high-performance CLI text search tool written in Rust, designed for speed and efficiency.

## Features

- **Parallel Processing**: Multi-threaded file searching using Rayon
- **Memory Mapping**: Efficient handling of large files with `memmap2`
- **Regex Support**: Full regex pattern matching with configurable options
- **Context Lines**: Show lines before/after matches
- **File Filtering**: Respect `.gitignore` and filter by file types
- **Colored Output**: Syntax highlighting with match emphasis
- **Fast Performance**: Optimized for speed with zero-cost abstractions

## Installation

### From Source

```bash
git clone https://github.com/yourusername/fastgrep
cd fastgrep
cargo build --release
```

### Global Installation

```bash
cargo install --path .
```

The binary will be available at `~/.cargo/bin/fastgrep` (ensure `~/.cargo/bin` is in your PATH).

## Usage

### Basic Search

```bash
fastgrep "pattern" file.txt
fastgrep "function" src/
```

### Common Options

```bash
# Case-insensitive search
fastgrep -i "todo" .

# Show line numbers
fastgrep -n "main" src/

# Show context (2 lines before/after)
fastgrep -C 2 "error" logs/

# Word boundaries only
fastgrep -w "test" .

# Literal string (no regex)
fastgrep -F "fn main()" src/

# Search specific file types
fastgrep -t rust "struct" .
```

### Advanced Usage

```bash
# Multiple context options
fastgrep -B 3 -A 1 "panic" src/     # 3 before, 1 after
fastgrep -C 2 "TODO" .              # 2 before and after

# File filtering
fastgrep --hidden "secret" .         # Include hidden files
fastgrep --no-ignore "debug" .       # Ignore .gitignore rules
fastgrep -T binary "text" .          # Exclude binary files

# Output control
fastgrep -l "error" logs/            # Files with matches only
fastgrep -L "success" logs/          # Files without matches
fastgrep -m 5 "import" src/          # Max 5 matches per file
```

## Command Reference

| Option | Description |
|--------|-------------|
| `-i, --ignore-case` | Case insensitive matching |
| `-w, --word-regexp` | Match whole words only |
| `-F, --fixed-strings` | Treat pattern as literal string |
| `-n, --line-number` | Show line numbers |
| `-C, --context NUM` | Show NUM lines before and after |
| `-B, --before-context NUM` | Show NUM lines before matches |
| `-A, --after-context NUM` | Show NUM lines after matches |
| `-m, --max-count NUM` | Stop after NUM matches per file |
| `-l, --files-with-matches` | Show only filenames with matches |
| `-L, --files-without-matches` | Show only filenames without matches |
| `-H, --with-filename` | Always print filenames |
| `-h, --no-filename` | Never print filenames |
| `-t, --type-add TYPE` | Include file type (rust, python, etc.) |
| `-T, --type-not TYPE` | Exclude file type |
| `--hidden` | Search hidden files/directories |
| `--no-ignore` | Don't respect .gitignore files |
| `--max-depth NUM` | Maximum directory depth |
| `--color WHEN` | Colorize output (auto/always/never) |

## Performance

FastGrep is optimized for performance with:

- **Parallel file processing** for multiple files
- **Memory-mapped I/O** for efficient file reading
- **Compiled regex patterns** cached for reuse
- **Release optimizations** including LTO and panic=abort
- **Smart file filtering** to skip binary files

### Benchmarks

*(Run your own benchmarks with `hyperfine` or `time`)*

```bash
# Compare with ripgrep and grep
hyperfine 'fastgrep "pattern" large_directory/' 'rg "pattern" large_directory/' 'grep -r "pattern" large_directory/'
```

## Building

### Requirements

- Rust 1.70+ (2021 edition)
- Cargo

### Development Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

## Dependencies

- `clap` - Command line parsing
- `regex` - Regular expression engine
- `rayon` - Parallel processing
- `memmap2` - Memory-mapped file I/O
- `ignore` - Gitignore and file type handling
- `colored` - Terminal color output
- `anyhow` - Error handling
- `atty` - TTY detection

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -am 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [ripgrep](https://github.com/BurntSushi/ripgrep)
- Built with Rust's excellent ecosystem