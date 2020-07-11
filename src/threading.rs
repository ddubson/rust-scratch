use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn threads_and_channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}