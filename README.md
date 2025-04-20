# no-incode-comments

A Rust procedural macro that allows you to keep documentation for your code in external Markdown files instead of inline doc comments.

## Purpose

This library solves the problem of maintaining documentation alongside your code by:

1. Letting you write documentation in proper Markdown files
2. Linking those files to your code through a simple attribute macro
3. Automatically importing the documentation at compile time

This approach has several benefits:
- Better organization of documentation
- Support for rich Markdown features
- Easier maintenance of extensive documentation
- Improved separation of concerns

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
no-incode-comments = "0.1.0"
```

## Usage

1. Create a Markdown file for your documentation (e.g., `docs/my_function.md`):

```markdown
# MyFunction

This is documentation for my_function.
It will be rendered as the doc comment for the function.

# AnotherSection
This section won't be included since we're only importing the "MyFunction" section.
```

2. Use the `external_doc` macro to import the documentation:

```rust
use no_incode_comments::external_doc;

#[external_doc(path = "docs/my_function.md", key = "MyFunction")]
pub fn my_function() {
    // Function implementation
}
```

The `path` parameter specifies the Markdown file location, and the `key` parameter indicates which section (by heading) to import.

## How It Works

The procedural macro:
1. Reads the specified Markdown file
2. Parses it to extract the section with the matching header (key)
3. Converts that section into Rust doc comments (`///`)
4. Applies those comments to your function

## Example

See the sample code in `main.rs` and the corresponding documentation in `docs/sample_file.md`.

## License

This project is available under the MIT License.
