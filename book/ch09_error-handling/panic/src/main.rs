fn main() {
    let cond = false;
    if cond {
        panic!("crash and burn");
    }

    let v = vec![1, 2, 3];
    v[99];
}
