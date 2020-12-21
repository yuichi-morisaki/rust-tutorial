use std::sync::mpsc; // multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel(); // transmitter, receiver

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        /*
        let val = String::from("Hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // error: borrow of moved value: `val`
        */
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("  more"),
            String::from("  message"),
            String::from("  for"),
            String::from("  you"),
        ];

        thread::sleep(Duration::from_millis(500));
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    /*
    let block = true;
    // let block = false;

    let received = if block {
        rx.recv().unwrap()
    } else {
        thread::sleep(Duration::from_millis(1));
        rx.try_recv().unwrap()
    };

    println!("Got: {}", received);
    */
}
