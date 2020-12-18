use std::fs::File;

#[allow(unused_variables)]
fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let f = File::open("hello.txt").unwrap();
}
