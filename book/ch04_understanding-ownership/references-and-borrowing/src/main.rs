fn main() {
    let mut s = String::from("hello");
    change(&mut s);

    let r1 = &mut s;
    // let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time
    println!("{}", r1);
    // println!("{}", r2);

    /*
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s; // cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}, {}, and {}", r1, r2, r3);
    */

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // r1 and r2 are no longer used after this point
    let r3 = &mut s;
    println!("{}", r3);

    let len = calculate_length(&s); // passing reference
    println!("The length of '{}' is {}.", s, len);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
