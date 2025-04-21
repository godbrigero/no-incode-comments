use no_incode_comments::external_doc;

#[external_doc(path = "examples/docs/function.md", key = "MyFunction")]
pub fn my_function() {
    println!("Hello from my_function!");
}

fn main() {
    my_function();
}
