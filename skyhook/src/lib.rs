use std::thread;

extern crate cancellation;

mod platforms;
pub mod types;

pub fn run() {
    platforms::linux::start().unwrap();

    thread::park();
}
