use std::{thread, time};

fn sleep_print(name: &str) {
    for i in 1..=3 {
        println!("{}: i={}", name, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    // no thread
    println!("--- no thread ---");
    sleep_print("no thread");

    // use thread
    println!("--- use thread ---");
    // thread 1
    thread::spawn(|| sleep_print("a"));
    // thread 2
    thread::spawn(|| sleep_print("b"));
    // main thread
    sleep_print("c");
}
