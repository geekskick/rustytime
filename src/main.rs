use std::time;
use std::io::Write;
use std::thread;
use std::sync::mpsc;
use chrono::prelude::*;

fn main() {
    let (tx,rx) = mpsc::channel();

    println!("[{}] - Starting, now waiting", Utc::now());

    thread::spawn(move || {
        thread::sleep(time::Duration::new(5,0));
        tx.send(true).unwrap();
    });

    let mut i = 0;

    while rx.try_recv().is_err() {
        let whirligig = String::from("-/|\\");
        print!("\r{}", whirligig.chars().nth(i).unwrap());
        std::io::stdout().flush().unwrap();
        i = i + 1;
        if whirligig.len() == i { 
            i = 0; 
        }
        thread::sleep(time::Duration::from_millis(100));
    }

    println!("\r[{}] - Hello, world!", Utc::now());
}
