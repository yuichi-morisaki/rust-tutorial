use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let p = Pair::new(3.5, 7.0);
    p.cmp_display();

    let pv = Pair::new(vec![1, 2], vec![3, 4, 5]);
    /* error: no method named `cmp_display` found
              for struct `Pair<Vec<{integer}>>`
    pv.cmp_display();
    */
}
