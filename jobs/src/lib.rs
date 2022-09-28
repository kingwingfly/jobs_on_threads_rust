use std::{thread, time::Duration};

pub fn job1() {
    thread::sleep(Duration::from_secs(2));
    println!("Run job1.")
}

pub fn job2() {
    thread::sleep(Duration::from_secs(3));
    println!("Run job2.")
}
