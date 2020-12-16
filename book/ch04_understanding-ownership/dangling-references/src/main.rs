fn main() {
    // let reference_to_nothing = dangle();
    let _reference_to_string = no_dangle();
}

/*
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
