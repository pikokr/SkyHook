use std::thread::Builder;

use winsafe::{
    co::WH,
    msg::wm,
    prelude::kernel_Hthread,
    prelude::{user_Hhook, Handle},
    HHOOK, HINSTANCE,
};

mod keyboard;
mod mouse;

use crate::types::{Error, Event};

use super::{message::process_message, CANCELLATION_TOKEN};

//#region Commons
pub(crate) static mut KBD_HOOK_ID: Option<HHOOK> = None;
pub(crate) static mut MOUSE_HOOK_ID: Option<HHOOK> = None;
static mut THREAD_ID: Option<u32> = None;
static mut CALLBACK: Option<fn(Event)> = None;

pub fn start(callback: fn(Event)) -> Result<(), Error> {
    unsafe {
        // return if hook is already set
        if KBD_HOOK_ID.is_some() {
            return Err(Error {
                message: "Hook cannot be started if the hook is already running.".into(),
            });
        }

        // assign callback
        CALLBACK = Some(callback);
    }

    let cancellation_token = match unsafe { &CANCELLATION_TOKEN } {
        Some(v) => v,
        None => {
            return Err(Error {
                message: "Cancellation token is None.".into(),
            })
        }
    };

    let thread = Builder::new().spawn(|| {
        let registered_keyboard_hook = HHOOK::SetWindowsHookEx(
            WH::KEYBOARD_LL,
            keyboard::hook_callback,
            Some(HINSTANCE::NULL),
            Some(0),
        );

        let registered_mouse_hook = HHOOK::SetWindowsHookEx(
            WH::MOUSE_LL,
            mouse::hook_callback,
            Some(HINSTANCE::NULL),
            Some(0),
        );

        unsafe {
            KBD_HOOK_ID = match registered_keyboard_hook {
                Ok(h) => Some(h),
                Err(err) => return Err(err),
            };
            MOUSE_HOOK_ID = match registered_mouse_hook {
                Ok(h) => Some(h),
                Err(err) => return Err(err),
            };

            let thread_id = winsafe::HTHREAD::GetCurrentThreadId();

            THREAD_ID = Some(thread_id);
        }

        process_message(cancellation_token);

        Ok(())
    });

    while let None = unsafe { KBD_HOOK_ID } {}

    if let Err(e) = thread {
        return Err(Error {
            message: format!("Failed to start hook thread: {:?}", e),
        });
    }

    Ok(())
}

pub fn stop() -> Result<(), Error> {
    let mut exists = false;

    unsafe {
        if let Some(hook_id) = KBD_HOOK_ID {
            exists = true;
            match HHOOK::UnhookWindowsHookEx(hook_id) {
                Ok(_) => (),
                Err(err) => {
                    return Err(Error {
                        message: format!("Could not stop the keyboard hook. {:?}", err),
                    })
                }
            };
        }

        if let Some(hook_id) = MOUSE_HOOK_ID {
            exists = true;
            match HHOOK::UnhookWindowsHookEx(hook_id) {
                Ok(_) => (),
                Err(err) => {
                    return Err(Error {
                        message: format!("Could not stop the mouse hook. {:?}", err),
                    })
                }
            };
        }

        if let Some(thread_id) = THREAD_ID {
            if let Err(e) = winsafe::PostThreadMessage(thread_id, wm::Close {}) {
                return Err(Error {
                    message: format!("Failed to send close message: {:?}", e),
                });
            }

            exists = true;
        }
    }

    if !exists {
        Err(Error {
            message: "Hook cannot be stopped before starting.".into(),
        })
    } else {
        Ok(())
    }
}
//#endregion
