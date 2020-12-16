fn main() {
    let x = 5;
    let mut y = x; // make a copy of the value in x
    println!("x is {}, and y is {}", x, y);

    y += 1;
    println!("x is {}, and y is {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // `move` in Rust instead of `shallow copy`

    // println!("s1 is {}", s1); // borrow of moved value: `s1`
    println!("s2 is {}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone(); // copy the heap data
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    let x = 5;
    let y = x; // `copy` is the default behavior in integer such as i32,
    let z = x.clone(); // so don't need call `clone`
    println!("x is {}, y is {}, and z is {}", x, y, z);
}
