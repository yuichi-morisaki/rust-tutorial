fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[allow(unused_variables)]
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

#[allow(unused_variables)]
fn longest3<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    // ---- Generic Lifetimes in Functions ----
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // ----
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // ----
    let result;
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    //println!("The longest string is {}", result);

    // ----
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result = longest2(string1.as_str(), string2.as_str());
}
