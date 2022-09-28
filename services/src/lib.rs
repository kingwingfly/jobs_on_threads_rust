use std::{sync::{mpsc::Sender, Arc, Mutex}, thread};
use chrono::prelude::*;
use job_threads::ThreadPool;
use jobs;
use std::time::Duration;

pub fn service(pool: &ThreadPool) {
    pool.excute(|| jobs::job1());
    pool.excute(|| jobs::job2());
}

fn meet_time_or_not() -> bool {
    let dt = Local::now();
    match dt.weekday() {
        Weekday::Sat | Weekday::Sun => return false,
        _ => true,
    };
    match dt.hour() {
        8 | 20 => true,
        _ => return false,
    };
    match dt.minute() {
        0 => true,
        _ => return false,
    };
    true
}

pub fn check_time(sender: Arc<Mutex<Sender<bool>>>) {
    loop {
        println!("Waiting...");
        if meet_time_or_not() {
            sender.lock().unwrap().send(true).unwrap();
            thread::sleep(Duration::from_secs(12*3600-300));
        }
        thread::sleep(Duration::from_secs(1));
    }
}