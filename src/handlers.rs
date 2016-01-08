#[macro_use]

use x11::xlib;
use window_system::WindowSystem;
use std::mem;
use x11::xinerama;

pub struct KeyPressHandler; 
pub struct MapRequestHandler;

impl KeyPressHandler {
    pub fn new() -> KeyPressHandler {
        return KeyPressHandler;
    }
}

impl MapRequestHandler {
    pub fn new() -> MapRequestHandler {
        return MapRequestHandler;
    }

    pub fn handle(&self, e: &mut xlib::XEvent, window_system: &WindowSystem) {
        let event = xlib::XMapRequestEvent::from(e);
//        xlib::XMapWindow(window_system.display,e.window);
        println!("Handling MapRequest");
    }
}
