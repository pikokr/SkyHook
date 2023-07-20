use std::time::SystemTime;

use crate::KeyCode;

#[derive(Debug)]
pub enum Event {
    KeyDown(EventData),
    KeyUp(EventData),
}

#[derive(Debug)]
pub struct EventData {
    pub code: KeyCode,
    pub key: i32,
    pub time: SystemTime,
}
