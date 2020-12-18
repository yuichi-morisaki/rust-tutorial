#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    // println!("r: {}", r); // `x` does not live long enough

    let x = 5;
    let r = &x;
    println!("r: {}", r);
}
