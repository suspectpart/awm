use x11::xlib;
use window_system::WindowSystem;

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

    pub fn handle(&self, e: xlib::XMapRequestEvent, window_system: &WindowSystem) {
        unsafe {
            xlib::XMapWindow(window_system.display,e.window);   
        }
        println!("Handling MapRequest");
    }
}
