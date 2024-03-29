use std::{
    collections::HashSet,
    ffi::{c_char, CStr},
};

use chrono::Local;

use crate::types::{Error, Event};

use self::keycode::raw_keycode_to_vk;

mod keycode;

extern "C" {
    fn start_macos_hook(callback: extern "C" fn(u16, bool)) -> *const c_char;
    fn stop_macos_hook() -> *const c_char;
    fn macos_hook_running() -> bool;
}

static mut CURRENT_CALLBACK: Option<fn(Event)> = None;

static mut PRESSED_KEYS: Option<HashSet<u16>> = None;

unsafe fn add_key(key: u16) -> bool {
    match PRESSED_KEYS.as_mut() {
        None => {
            let mut hs = HashSet::<u16>::new();

            hs.insert(key);

            PRESSED_KEYS = Some(hs);

            return true;
        }
        Some(keys) => {
            return keys.insert(key);
        }
    }
}

unsafe fn remove_key(key: u16) {
    if let Some(keys) = PRESSED_KEYS.as_mut() {
        keys.remove(&key);
    }
}

extern "C" fn native_callback(key: u16, down: bool) {
    unsafe {
        if down {
            if !add_key(key) {
                return;
            }
        } else {
            remove_key(key);
        }

        if let Some(cb) = CURRENT_CALLBACK {
            cb(Event {
                time: Local::now().naive_local(),
                data: match down {
                    true => crate::types::EventData::KeyPress(raw_keycode_to_vk(key), key),
                    false => crate::types::EventData::KeyRelease(raw_keycode_to_vk(key), key),
                },
            })
        }
    }
}

pub fn start(#[allow(unused_variables)] callback: fn(Event)) -> Result<(), Error> {
    unsafe {
        CURRENT_CALLBACK = Some(callback);

        let result = start_macos_hook(native_callback);

        if result.is_null() {
            return Ok(());
        }

        Err(Error {
            message: CStr::from_ptr(result)
                .to_str()
                .expect("Unable to convert pointer to string")
                .into(),
        })
    }
}

pub fn stop() -> Result<(), Error> {
    unsafe {
        let result = stop_macos_hook();

        if result.is_null() {
            return Ok(());
        }

        Err(Error {
            message: CStr::from_ptr(result)
                .to_str()
                .expect("Unable to convert pointer to string")
                .into(),
        })
    }
}

pub fn is_running() -> bool {
    unsafe { macos_hook_running() }
}
