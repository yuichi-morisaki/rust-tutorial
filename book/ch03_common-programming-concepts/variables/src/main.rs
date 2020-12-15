const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Constant MAX_POINTS is: {}", MAX_POINTS);

    // Immutability
    let x = 5;
    println!("The value of x is: {}", x);

    /*  cannot assign twice to immutable variable `x`

        x = 6;
        println!("The value of x is: {}", x);
    */

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // Shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {}", z);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The number of spaces for indent is: {}", spaces);

    /*  mismatched types - we can't mutate a variable's type

        let indent = "    ";
        indent = indent.len();  // expected `&str`
    */
}
