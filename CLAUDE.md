# CLAUDE.md - AI Assistant Guide for code2prompt

This document provides a comprehensive guide for AI assistants working with the code2prompt codebase.

## Project Overview

**code2prompt** is a Rust command-line tool that converts entire codebases into single LLM-friendly prompts. It generates well-formatted Markdown prompts with source tree visualization, supports customizable Handlebars templates, token counting, and git integration.

- **Version**: 2.0.0
- **Language**: Rust (Edition 2021)
- **License**: MIT
- **Repository**: https://github.com/mufeedvh/code2prompt

## Quick Commands

```bash
# Build
cargo build
cargo build --release

# Run tests
cargo test

# Run the tool
cargo run -- <path> [options]

# Example: Generate prompt from current directory
cargo run -- . --tokens

# Example with template
cargo run -- . --template write-github-readme
```

## Project Structure

```
code2prompt/
├── src/                          # Source code
│   ├── main.rs                   # Entry point, CLI orchestration
│   ├── lib.rs                    # Library exports
│   ├── cli.rs                    # CLI argument definitions (clap)
│   ├── path.rs                   # Directory traversal (jwalk)
│   ├── template.rs               # Handlebars template engine
│   ├── filter.rs                 # File filtering (glob patterns)
│   ├── git.rs                    # Git operations (git2)
│   ├── token.rs                  # Token counting (tiktoken-rs)
│   ├── config.rs                 # Persistent configuration storage
│   ├── default_template.hbs      # Embedded default template
│   └── comment_remover/          # Comment stripping by language
│       ├── mod.rs                # Trait and factory
│       ├── rust_comment_remover.rs
│       ├── python_comment_remover.rs
│       └── javascript_comment_remover.rs
├── templates/                    # Handlebars templates
│   ├── default_template.hbs
│   ├── document-the-code.hbs
│   ├── find-security-vulnerabilities.hbs
│   ├── clean-up-code.hbs
│   ├── fix-bugs.hbs
│   ├── improve-performance.hbs
│   ├── refactor.hbs
│   ├── write-git-commit.hbs
│   ├── write-github-pull-request.hbs
│   ├── write-github-readme.hbs
│   └── *-ctf-solver.hbs          # CTF challenge templates
├── tests/                        # Integration tests
│   ├── integration_test.rs
│   ├── test_filter.rs
│   ├── git_test.rs
│   └── template_test.rs
├── Cargo.toml                    # Project manifest
├── build.rs                      # Build script (template installation)
└── .github/workflows/            # CI/CD configurations
```

## Module Responsibilities

| Module | File | Purpose |
|--------|------|---------|
| `cli` | `src/cli.rs` | Defines all CLI arguments using clap derive macros |
| `path` | `src/path.rs` | Directory traversal, file tree building, file processing |
| `template` | `src/template.rs` | Handlebars setup, rendering, clipboard, file output |
| `filter` | `src/filter.rs` | Glob pattern matching for include/exclude filtering |
| `git` | `src/git.rs` | Git diff, branch comparison, log retrieval |
| `token` | `src/token.rs` | Token counting with multiple tokenizer support |
| `config` | `src/config.rs` | Save/retrieve last used arguments per directory |
| `comment_remover` | `src/comment_remover/` | Language-specific comment stripping |

## CLI Options Reference

```
code2prompt <PATH> [OPTIONS]

Arguments:
  <PATH>                    Path to the codebase directory

Options:
  --include <PATTERNS>      Comma-separated patterns to include
  --exclude <PATTERNS>      Comma-separated patterns to exclude
  --include-priority        Include files when include/exclude conflict
  --exclude-from-tree       Exclude from source tree visualization
  --tokens                  Display token count
  -c, --encoding <TYPE>     Tokenizer: cl100k (default), p50k, p50k_edit, r50k, gpt2
  -o, --output <FILE>       Output file path
  -d, --diff                Include git diff (unstaged changes)
  --git-diff-branch <A,B>   Git diff between two branches
  --git-log-branch <A,B>    Git log between two branches
  -l, --line-number         Add line numbers to code
  --no-codeblock            Disable markdown code blocks
  --relative-paths          Use relative paths instead of absolute
  --no-clipboard            Disable clipboard copy
  -t, --template <PATH>     Custom Handlebars template
  --json                    Output as JSON
  --remove-comments         Strip comments from source code
```

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing with derive macros |
| `handlebars` | Templating engine |
| `jwalk` | Parallel directory traversal |
| `git2` | Git operations (vendored OpenSSL/libgit2) |
| `tiktoken-rs` | OpenAI token counting |
| `regex` | Comment removal patterns |
| `ignore` | .gitignore parsing |
| `arboard` | Clipboard access |
| `indicatif` | Progress bars/spinners |
| `colored` | Terminal colors |
| `anyhow` | Error handling |
| `serde_json` | JSON serialization |

## Code Conventions

### Error Handling
- Use `anyhow::Result<T>` for all fallible functions
- Add context with `.context("description")` on `?` operations
- Log errors with `error!()` macro before returning

### Documentation
- All public functions have doc comments (`///`)
- Include `# Arguments`, `# Returns`, `# Errors` sections
- Add `# Example` for complex functions

### Logging
- `debug!()` - Detailed internal state
- `info!()` - Progress updates
- `error!()` - Failure conditions

### Output Formatting
- Success: `[✓]` in green
- Info: `[i]` in blue
- Use `colored` crate methods: `.green()`, `.yellow()`, `.blue()`, `.red()`

### Pattern Usage
```rust
// Strategy pattern for comment removers
pub trait CommentRemover {
    fn remove_comments(&self, code: &str) -> (String, usize);
}

// Factory function
pub fn get_comment_remover(extension: &str) -> Option<Box<dyn CommentRemover>>
```

## Template System

### Built-in Template Variables
- `absolute_code_path` - Full path to the codebase
- `source_tree` - Visual directory tree
- `files` - Array of file objects
- `git_diff` - Unstaged changes
- `git_diff_branch` - Branch comparison diff
- `git_log_branch` - Branch comparison log

### File Object Properties
- `path` - File path
- `extension` - File extension
- `code` - File contents
- `comments_removed` - Count of removed comment lines

### User-Defined Variables
Templates can include `{{variable_name}}` placeholders that are interactively prompted during execution.

### Template Location
- Project templates: `./templates/`
- User templates: `~/.code2prompt/templates/`

## Testing

### Running Tests
```bash
# All tests
cargo test

# Specific test file
cargo test --test integration_test
cargo test --test test_filter
cargo test --test git_test
cargo test --test template_test

# With output
cargo test -- --nocapture
```

### Test Patterns
- Use `tempfile` crate for isolated test directories
- Use `assert_cmd` for CLI integration testing
- Initialize logger once with `once_cell::sync::Once`

## Build Configuration

### Release Profile (Cargo.toml)
```toml
[profile.release]
lto = "thin"           # Thin link-time optimization
panic = 'abort'        # No unwinding
codegen-units = 1      # Single codegen unit
```

### Build Script (build.rs)
Copies templates to `~/.code2prompt/templates/` during build.

### Supported Targets
- Linux: x86_64, i686, aarch64, arm
- macOS: x86_64, aarch64 (Apple Silicon)
- Windows: x86_64, i686 (MSVC and GNU)

## Configuration Storage

User preferences are stored in `~/.code2prompt_configs.json`:
- Saves last used arguments per directory
- Automatically restores settings on next invocation
- Managed by `src/config.rs`

## Common Development Tasks

### Adding a New CLI Option
1. Add field to `Cli` struct in `src/cli.rs`
2. Add clap attributes for parsing
3. Update `ConfigStore` serialization if needed
4. Handle in `main.rs` or appropriate module

### Adding a New Template
1. Create `.hbs` file in `templates/`
2. Use built-in variables: `{{absolute_code_path}}`, `{{source_tree}}`, `{{#each files}}`
3. Template auto-discovered by name

### Adding Comment Remover for New Language
1. Create `src/comment_remover/<lang>_comment_remover.rs`
2. Implement `CommentRemover` trait
3. Add to factory in `src/comment_remover/mod.rs`
4. Map file extensions in `get_comment_remover()`

### Adding a New Tokenizer
1. Update `get_tokenizer()` in `src/token.rs`
2. Update `get_model_info()` for display info
3. Document in CLI help text

## Important Implementation Notes

### Clipboard Workaround
The `arboard` crate requires a 200ms sleep after clipboard operations due to timing issues (see code comments in `src/template.rs`).

### Git Operations
Uses vendored `git2` with OpenSSL and libgit2 to avoid system dependency issues.

### .gitignore Respect
Uses `ignore` crate to automatically exclude files matching `.gitignore` patterns during traversal.

### Path Handling
- Always canonicalize paths for symlink handling
- Support both relative and absolute path output
- Use `std::path::Path` for cross-platform compatibility

## Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run -- .

# Enable all logging
RUST_LOG=trace cargo run -- .
```

## File Reading Order for New Contributors

1. `README.md` - High-level features and usage
2. `src/cli.rs` - Understand all CLI options
3. `src/main.rs` - Control flow and orchestration
4. `src/path.rs` - Core traversal logic
5. `src/template.rs` - Template system
6. Feature modules as needed (git, token, filter)
