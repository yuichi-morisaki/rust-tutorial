fn main() {
    {
        // cannot find value `s` in this scope
        // println!("{}", s);

        let s = "hello"; // `s` is valid from here until end of this block `}`

        println!("{}", s);
    }

    // cannot find value `s` in this scope
    // println!("{}", s);
}
