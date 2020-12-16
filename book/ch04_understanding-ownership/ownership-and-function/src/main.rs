fn main() {
    // ----
    // passing a value to a function

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // borrow of moved value: `s`

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    // ----
    // returning a value from a function

    let s1 = gives_ownership();
    println!("s1 is {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("s2 is {}", s2); // borrow of moved value: `s2`
    println!("s3 is {}", s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // move out to the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
