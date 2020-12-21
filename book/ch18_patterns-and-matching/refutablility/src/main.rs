#[allow(unused_variables)]
fn main() {
    let some_option_value = Some(5);

    /*  error:
     *      refutable pattern in local binding:
     *      `None` not covered
     */
    // let Some(x) = some_option_value;

    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    /*  warning:
     *      irrefutable if-let pattern
     */
    if let x = 5 {
        println!("{}", x);
    }
}
