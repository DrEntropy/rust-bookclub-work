use std::fs::File;

fn main() {
    let file_result = File::open("hello_42.txt");  // file doesnt exist
    let file = file_result.expect("As expected, hello_42 doesnt exist. ");  // panic if file_result is Err
}
