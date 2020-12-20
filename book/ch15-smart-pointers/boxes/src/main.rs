use boxes::List::{Cons, Nil};

#[allow(unused_variables)]
fn main() {
    let b = Box::new(5);
    // println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let list = Cons(3, Box::new(Nil));
    let list = Cons(2, Box::new(list));
    let list = Cons(1, Box::new(list));

    // ----

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // assert_eq!(5, y); // error
    assert_eq!(5, *y); // need to dereference

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, y); // error
    assert_eq!(5, *y);
}
