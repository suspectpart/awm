use std::ptr;
use xlib::{ Display, Window, Screen };
use xlib::{ XOpenDisplay, XDefaultScreenOfDisplay, XRootWindowOfScreen };

pub struct WindowSystem {
    pub display:  *mut Display,
    pub root: Window,
}

impl WindowSystem {
   pub fn new() -> WindowSystem {
        unsafe {
            let display = XOpenDisplay(ptr::null_mut()); // Null Pointer means "just use env variable $DISPLAY
            let screen = XDefaultScreenOfDisplay(display);
            let root = XRootWindowOfScreen(screen);

            WindowSystem {
                display: display,
                root: root,
            }
        }
    }
}
