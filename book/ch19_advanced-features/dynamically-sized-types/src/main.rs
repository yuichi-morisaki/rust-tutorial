fn main() {
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
}

fn generic<T>(t: T) {
    // --snip--
}

fn generic<T: Sized>(t: T) {
    // --snip--
}

fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
