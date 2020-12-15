fn five() -> i32 {
    5
}

fn main() {
    let x = five(); // statement

    // block, {...}, is an expression
    let _y = {
        let x = 3;
        x + 1 // not ending with semi-colon `;`
    };

    /* error: expected expression, found statement
    let x = (let y = 6);
    */

    let y = plus_one(x);

    another_function(x, y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
