use std::ptr;
use std::cell;
use x11::xlib::*;

pub struct WindowInfo {
    pub height: i32,
    pub width: i32,
}

pub struct WindowSystem {
    pub display: *mut Display,
    pub root: Window,
    pub info: WindowInfo,
    pub count: cell::Cell<u32>,
    
}

impl WindowSystem {
   pub fn new() -> WindowSystem {
        unsafe {
            let display = XOpenDisplay(ptr::null_mut()); // Null Pointer means "just use env variable $DISPLAY
            let screen = XDefaultScreenOfDisplay(display);
            let root = XRootWindowOfScreen(screen);

            let height = (*screen).height;
            let width= (*screen).width;
            
            WindowSystem{
                display: display,
                root: root,
                info: WindowInfo {
                    height: height,
                    width: width,
                },
                count: cell::Cell::new(0),
            }
        }
    }
}
