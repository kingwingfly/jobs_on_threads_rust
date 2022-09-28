use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use job_threads::ThreadPool;

fn main() {
    let pool = ThreadPool::new(2);
    let (sender, receiver) = mpsc::channel();
    let sender = Arc::new(Mutex::new(sender));
    let receiver = Mutex::new(receiver);
    thread::spawn(move || loop {
        services::check_time(Arc::clone(&sender))
    });
    loop {
        if receiver.lock().unwrap().recv().unwrap() {
            services::service(&pool);
        }
    }
}
