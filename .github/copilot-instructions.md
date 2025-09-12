# Egrapsa - Text to LaTeX Converter

Egrapsa is a Rust CLI application that converts text from online libraries (currently Scaife Perseus Digital Library with Roman and Greek texts) to LaTeX files for PDF generation. The typography style emulates 17th and 18th century printed books with features like strange ligatures, long s, text ornaments, and catch words.

**ALWAYS reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the information here.**

## Working Effectively

### Prerequisites and Setup
- Rust toolchain 1.89.0 or later is required and already available
- Internet connectivity is required for the application to fetch texts from external sources
- The repository structure includes:
  - `src/` - Rust source code (11 .rs files total)
  - `configs/` - Example JSON configuration files for different works
  - `Cargo.toml` - Rust project configuration

### Build Commands
**NEVER CANCEL these build commands - they take significant time:**

- **Clean release build**: `cargo clean && cargo build --release`
  - Takes approximately 30-50 seconds to complete
  - NEVER CANCEL: Set timeout to 180+ seconds minimum  
  - Compiles all dependencies and application in optimized release mode
  - Use for first-time builds or after major dependency changes

- **Incremental release build**: `cargo build --release`
  - Takes approximately 1-5 seconds for incremental builds
  - Takes 30+ seconds for clean builds
  - NEVER CANCEL: Set timeout to 120+ seconds minimum
  - Required before running the application

- **Development build**: `cargo build`
  - Takes approximately 1-2 seconds for incremental builds  
  - Faster unoptimized build for development and testing

### Test Commands
**NEVER CANCEL test commands:**

- **Run all tests**: `cargo test`
  - Takes approximately 17 seconds to complete  
  - NEVER CANCEL: Set timeout to 60+ seconds minimum
  - Runs 15 unit tests covering text processing, LaTeX formatting, and source parsing
  - All tests should pass in a healthy codebase

### Linting and Code Quality
Always run these before committing changes:

- **Lint with Clippy**: `cargo clippy`
  - Takes approximately 7-8 seconds
  - Reports warnings about code quality, performance, and style
  - Current codebase has some acceptable warnings (23 warnings total)

- **Format code**: `cargo fmt`
  - Formats code according to Rust conventions
  - Use `cargo fmt -- --check` to verify formatting without making changes
  - Current codebase has some formatting issues that should be addressed

- **Check formatting**: `cargo fmt -- --check`
  - Returns exit code 1 if formatting changes are needed
  - Returns exit code 0 if code is properly formatted

### Running the Application

**Basic usage:**
```bash
./target/release/egrapsa --config-path <CONFIG_PATH> --output-path <OUTPUT_PATH>
```

**Example with provided configs:**
```bash
# Must build first with cargo build --release
./target/release/egrapsa --config-path configs/phalaris.json --output-path output.tex
```

**Important notes:**
- Application requires internet access to fetch texts from Scaife Perseus Digital Library
- Without internet access, application will panic with "ConnectionError"
- Processing time depends on the size of the work being converted
- Output is a LaTeX file that can be compiled to PDF

### Application Help and Version
```bash
./target/release/egrapsa --help     # Show usage information
./target/release/egrapsa --version  # Show version information
```

## Validation and Testing

## Validation and Testing

### Complete Validation Workflow
After making changes to the codebase, ALWAYS run this complete validation sequence:

```bash
# 1. Clean and build (30-50 seconds) - NEVER CANCEL
cargo clean && cargo build --release

# 2. Run all tests (17 seconds) - NEVER CANCEL  
cargo test

# 3. Run linting (7 seconds)
cargo clippy

# 4. Check formatting (immediate)
cargo fmt -- --check

# 5. Verify application help works
./target/release/egrapsa --help
./target/release/egrapsa --version

# 6. Test basic application functionality (expects ConnectionError)
timeout 10 ./target/release/egrapsa --config-path configs/phalaris.json --output-path /tmp/test.tex
```

### Expected Validation Results
- **Build**: Should complete successfully with 1 compiler warning about lifetime syntax
- **Tests**: All 15 tests should pass in ~17 seconds  
- **Clippy**: Currently shows 23 warnings (acceptable for current codebase)
- **Format check**: Currently fails with exit code 1 (formatting issues exist)
- **Help/Version**: Should display correctly
- **App run**: Should panic with "ConnectionError" when offline (expected behavior)

### Manual Testing with Internet Access
When internet connectivity is available, test these scenarios:

1. **Small work conversion**: 
   ```bash
   ./target/release/egrapsa --config-path configs/phalaris.json --output-path phalaris.tex
   ```

2. **Multiple works**: 
   ```bash  
   ./target/release/egrapsa --config-path configs/example_config_2.json --output-path cicero.tex
   ```

3. **Large compilation**:
   ```bash
   ./target/release/egrapsa --config-path configs/example_config.json --output-path lucian_complete.tex
   ```

4. **Verify LaTeX output**:
   - Output files should contain LaTeX markup
   - Files should include `\documentclass[a5paper,12pt]{book}`
   - Files should include `\begin{document}`, `\end{document}`  
   - Files should include appropriate language packages (`babel`, font packages)
   - Text should include special formatting for classical texts
   - Greek texts should use `TheanoOldStyle` font and `greek.polutoniko` babel
   - Latin texts should use `kpfonts` with `oldstyle, veryoldstyle` options

### Understanding Configuration Files
Configuration files in `configs/` directory follow this JSON structure:
```json
{
    "name": "Work Name",
    "formatter_type": "Latex", 
    "formatter_config": {
        "title": "LaTeX Document Title",
        "author": "Author Name",
        "catchwords": false,
        "ref_numbers": true,
        "footnotes": true,
        "language": "Greek" | "Latin"
    },
    "source_type": "Scaife",
    "work_infos": [
        {
            "title": "Work Title",
            "alt_title": "Alternative Title",
            "identifier": "urn:cts:greekLit:..." 
        }
    ]
}
```

Key configuration options:
- `catchwords`: Enable 17th/18th century style catch words
- `ref_numbers`: Include reference numbers in margins
- `footnotes`: Enable footnote generation
- `language`: Determines font and babel packages used

### Timeout Configuration for CI/CD
When setting up automated workflows:
- Build commands: Set timeout to 180+ seconds (3+ minutes)
- Test commands: Set timeout to 120+ seconds (2+ minutes)  
- Lint commands: Set timeout to 60+ seconds (1+ minute)

## Repository Navigation

### Key Source Files
- `src/main.rs` - Application entry point and CLI argument parsing
- `src/config.rs` - Configuration file parsing and work definition
- `src/text.rs` - Core text processing, formatting, and LaTeX generation
- `src/formatters/latex.rs` - LaTeX-specific formatting logic
- `src/text_sources/scaife.rs` - Scaife Perseus Digital Library integration
- `src/text_sources/mod.rs` - Text source abstractions

### Configuration Files
- `configs/example_config.json` - Large example with Lucian's complete works
- `configs/example_config_2.json` - Smaller example with Cicero's works  
- `configs/phalaris.json` - Single work example (good for testing)
- `configs/*.json` - Additional pre-configured examples for various classical works

### Development Files
- `Cargo.toml` - Project dependencies and metadata
- `Cargo.lock` - Locked dependency versions
- `TODO` - Development roadmap and planned features

## Common Tasks and Troubleshooting

### Quick Command Reference
```bash
# Complete validation workflow (run after making changes):
cargo clean && cargo build --release  # ~32s, NEVER CANCEL
cargo test                            # ~15s, NEVER CANCEL  
cargo clippy                          # ~5s
cargo fmt -- --check                 # immediate, exit code 1 = needs formatting
./target/release/egrapsa --help       # verify application works

# Development workflow:
cargo build                           # ~1-2s incremental
cargo test                           # ~15s, NEVER CANCEL
cargo clippy                         # ~5s  
cargo fmt                            # apply formatting fixes

# Application usage:
./target/release/egrapsa --config-path configs/phalaris.json --output-path output.tex
```

### Adding New Dependencies
1. Edit `Cargo.toml` to add dependency
2. Run `cargo build` to download and compile
3. Update any relevant code
4. Run full test suite with `cargo test`

### Code Style and Formatting Issues
- Current codebase has formatting inconsistencies
- Run `cargo fmt` to auto-fix most issues
- Some manual formatting may be required for complex expressions
- Clippy warnings are generally acceptable but should be reviewed

### Network-Related Issues
- Application panics with "ConnectionError" when offline
- This is expected behavior - application requires internet access
- Scaife Perseus Digital Library must be accessible
- Consider implementing better error handling for production use

### Performance Considerations
- Release builds are significantly faster than debug builds for text processing
- Text processing can be memory-intensive for large works (e.g., complete works collections)
- LaTeX output files can become very large for comprehensive works
- Network requests to Scaife may take time depending on work size and connection speed

### Build Time Expectations (Based on Validation)
- **Clean build**: 30-35 seconds (release), NEVER CANCEL before 60 seconds
- **Incremental build**: 1-5 seconds (release), 1-2 seconds (debug)
- **Test suite**: 15-17 seconds, NEVER CANCEL before 30 seconds
- **Clippy lint**: 5-7 seconds
- **Format check**: Immediate (< 1 second)

## Repository Information Cache

### Repository Root Contents
```
.
├── .git/               # Git repository data
├── .gitignore         # Git ignore rules  
├── Cargo.lock         # Locked dependencies
├── Cargo.toml         # Project configuration
├── LICENSE            # Project license
├── README.md          # Basic project documentation
├── TODO               # Development roadmap
├── configs/           # Example configuration files
└── src/              # Rust source code
```

### Config Directory Contents
```
configs/
├── cicero.json            # Cicero's works
├── example_config.json    # Lucian's complete works (large)
├── example_config_2.json  # Cicero's works (smaller)
├── homer.json            # Homer's works
├── lucian-4.json         # Lucian volume 4
├── lucian-5.json         # Lucian volume 5  
├── lucian-6.json         # Lucian volume 6
└── phalaris.json         # Single work (good for testing)
```

### Dependencies Summary
Key dependencies from Cargo.toml:
- `clap` 4.5.3 - Command line argument parsing
- `serde` 1.0.210 - JSON serialization/deserialization  
- `serde_json` 1.0.128 - JSON handling
- `ureq` 2.9.6 - HTTP client for fetching texts
- `quick-xml` 0.31.0 - XML parsing for Scaife responses
- `regex` 1.11.0 - Text processing and pattern matching
- `itertools` 0.13.0 - Iterator utilities
- `thiserror` 1.0.58 - Error handling

This information is current as of the repository state and should be referenced to avoid redundant exploration of the codebase.