use crate::types::{Error, Event};

use self::methods::LinuxHookMethod;

mod inputdev;
mod methods;
mod xinput;

static mut SELECTED_METHOD: LinuxHookMethod = LinuxHookMethod::Unset;

pub fn start(callback: fn(Event)) -> Result<(), Error> {
    if let LinuxHookMethod::Unset = unsafe { &SELECTED_METHOD } {
        if let Err(_) = inputdev::start(callback) {
            xinput::start(callback)?;
            unsafe {
                SELECTED_METHOD = LinuxHookMethod::XInput;
            }
        } else {
            unsafe {
                SELECTED_METHOD = LinuxHookMethod::InputDev;
            }
        }

        Ok(())
    } else {
        Err(Error {
            message: "Hook is already running".into(),
        })
    }
}

#[allow(dead_code)]
pub fn stop() -> Result<(), Error> {
    match unsafe { &SELECTED_METHOD } {
        LinuxHookMethod::InputDev => {
            inputdev::stop()?;
            unsafe {
                SELECTED_METHOD = LinuxHookMethod::Unset;
            }
        }
        LinuxHookMethod::XInput => {
            xinput::stop()?;
            unsafe {
                SELECTED_METHOD = LinuxHookMethod::Unset;
            }
        }
        _ => {
            return Err(Error {
                message: "The hook method is not set properly".into(),
            })
        }
    };

    Ok(())
}

pub fn is_running() -> bool {
    match unsafe { &SELECTED_METHOD } {
        LinuxHookMethod::InputDev => inputdev::is_running(),
        LinuxHookMethod::XInput => xinput::is_running(),
        _ => false,
    }
}
