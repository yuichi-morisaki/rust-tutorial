fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");
    /* type annotation needed
    let _guess = "42".parse().expect("Not a number!");
    */

    // Scalar Types
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    let _t = true;
    let _f: bool = false;

    let _c = 'z';
    let _z: char = '漢'; // 4 bytes

    // Compound Types
    let _tup: (i32, f64, u8) = (500, 6.4, 1); // tuple
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    let _a = [1, 2, 3, 4, 5]; // array
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5]; // [3, 3, 3, 3, 3]

    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];

    let index = 10;
    let element = a[index]; // index out of bounds
    println!("The value of element is: {}", element);
}
