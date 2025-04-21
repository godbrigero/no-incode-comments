use no_incode_comments::external_doc;

#[external_doc(path = "examples/docs/struct.md", key = "MyStruct")]
pub struct MyStruct {
    pub field1: i32,
    pub field2: String,
}

fn main() {
    let my_struct = MyStruct {
        field1: 42,
        field2: "Hello".to_string(),
    };
    println!(
        "MyStruct: field1 = {}, field2 = {}",
        my_struct.field1, my_struct.field2
    );
}
