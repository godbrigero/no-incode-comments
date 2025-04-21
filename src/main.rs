use no_incode_comments::external_doc;

pub fn main() {
    custom_function()
}

#[external_doc(path = "docs/sample_file.md", key = "custom_function")]
pub fn custom_function() {
    println!("Hello, world!");
}

#[external_doc(path = "docs/sample_file.md", key = "MyStruct")]
pub struct MyStruct {
    pub name: String,
    pub age: u32,
}
