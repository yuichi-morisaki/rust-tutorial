fn bar() -> ! {
    loop {}
}

fn main() {
    let guess = String::from("Hello");

    /*  error: `match` arms have incompatible types
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => "hello",
    };
    */

    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // returns !
        };
    }
}

impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
