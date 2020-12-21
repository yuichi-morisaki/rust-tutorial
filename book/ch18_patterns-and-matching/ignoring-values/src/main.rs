fn main() {
    // Ignoring an Entire Value with _
    {
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }

        foo(3, 4);
    }

    // Ignoring Parts of a Value with a Nested _
    {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {:?}", setting_value);

        // another case
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth);
            }
        }
    }

    // Ignoring an Unused Variable by Starting Its Name with _
    {
        let _x = 5;
        // let y = 10; // warning: unused variable

        // ----

        let s = Some(String::from("Hello"));

        if let Some(_s) = s {
            println!("found a string");
        }

        // println!("{:?}", s); // error: borrow of partially moved value: `s`

        // ----

        let s = Some(String::from("Hello"));

        if let Some(_) = s {
            println!("found a string");
        }

        println!("{:?}", s);
    }

    // Ignoring Remaining Parts of a Value with ..
    {
        #[allow(dead_code)]
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        // ----

        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => println!("Some numbers: {}, {}", first, last),
        }

        /*  error:
                `..` can only be used once per tuple pattern

        match numbers {
            (.., second, ..) => println!("Some numbers: {}", second),
        }
        */
    }
}
