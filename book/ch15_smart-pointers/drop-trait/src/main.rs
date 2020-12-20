struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    println!("CustomSmartPointer created.");
    // c.drop(); // error: explicit use of destructor method
    drop(c); // std::mem::drop (prelude)
    println!("CustomSmartPointer dropped before the end of main.");

    /*
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    */
}
