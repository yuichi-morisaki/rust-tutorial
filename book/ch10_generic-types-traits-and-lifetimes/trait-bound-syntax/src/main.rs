pub trait Summary {
    fn summarize(&self) -> String;
}

pub fn notify(item: &impl Summary) {
    item.summarize();
}

pub fn notify<T: Summary>(item: &T) {
    item.summarize();
}

pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    item1.summarize();
    item2.summarize();
}

pub fn notify<T: Summary>(item1: &T, item2: &T) {
    item1.summarize();
    item2.summarize();
}

pub fn notify(item: &(impl Summary + Display)) {
    item.summarize();
    format!("{}", item)
}

pub fn notify<T: Summary + Display>(item: &T) {
    item.summarize();
    format!("{}", item)
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // ...
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}

fn main() {
    println!("Hello, world!");
}
