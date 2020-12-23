/* error
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
*/

fn returns_closure2() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn returns_closure3() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

#[allow(unused_must_use)]
fn main() {
    // returns_closure();
    returns_closure2();
    returns_closure3();
}
