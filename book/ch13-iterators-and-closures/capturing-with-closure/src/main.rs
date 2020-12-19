fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    /*
    fn equal_to_x(z: i32) -> bool {
        z == x // error:
               // can't capture dynamic environment in a fn item
    }
    */

    let y = 4;

    assert!(equal_to_x(y));

    // ----

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    /*
    println!("can't use x here: {:?}", x); // error
                                           // borrow of moved value: `x`
    */

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
