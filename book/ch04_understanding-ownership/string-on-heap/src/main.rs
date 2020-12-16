fn main() {
    let _s = "hello"; // string literal

    {
        let mut s = String::from("hello"); // request memory for `s`
        s.push_str(", world!");
        println!("{}", s);
    } // call `drop` to free memory for `s`
}
