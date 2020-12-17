#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    // creating a new String
    {
        let mut s = String::new();

        let data = "initial contents";
        let s = data.to_string();

        // the method also works on a literal directly
        let s = "initial contents".to_string();

        let s = String::from("initial contents");
    }

    // updating a String
    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);

        let mut s = String::from("lo");
        s.push('l');
        println!("{}", s);

        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // s1 has been moved here

        // println!("s1 is '{}'", s1); // error
        println!("s2 is '{}'", s2);
        println!("s3 is '{}'", s3);

        let s1 = String::from("tic");
        let s2 = String::from("toc");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("{}", s);

        let s1 = String::from("tic");
        let s2 = String::from("toc");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{}", s1);
    }

    // indexing into Strings
    {
        let s1 = String::from("hello");
        // let h = s1[0]; // error: `String` cannot be indexed by `{integer}`

        let hello = String::from("Hola");
        println!("{}", hello.len()); // 4
        let hello = String::from("Здравствуйте");
        println!("{}", hello.len()); // 24
        let hello = String::from("こんにちは");
        println!("{}", hello.len()); // 15

        let hello = String::from("Здравствуйте");
        println!("{}", hello);
        let s = &hello[0..4];
        println!("{}", s);
        // let s = &hello[0..1];   // panic
    }

    // iterating over Strings
    {
        for c in "नमस्ते".chars() {
            println!("{}", c);
        }

        for b in "नमस्ते".bytes() {
            println!("{}", b);
        }
    }
}
