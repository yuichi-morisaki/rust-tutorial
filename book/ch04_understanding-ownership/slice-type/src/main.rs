fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("the first word is: {}", word);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("'{}' and '{}'", hello, world);
}

fn first_word(s: &str /*&String*/) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

/*
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
*/
