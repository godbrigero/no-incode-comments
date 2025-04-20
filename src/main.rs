use no_incode_comments::external_doc;

pub use no_incode_comments::*;

pub fn main() {
    custom_function()
}

#[external_doc(path = "docs/sample_file.md", key = "custom_function")]
pub fn custom_function() {
    println!("Hello, world!");
}
