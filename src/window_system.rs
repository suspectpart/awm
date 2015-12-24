use std::ptr;
use xlib::{ Display, Window };
use xlib::{ XOpenDisplay, XDefaultScreenOfDisplay, XRootWindowOfScreen };

pub struct WindowSystem {
    display:  *mut Display,
    root: Window
}
