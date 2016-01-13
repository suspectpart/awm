use x11::xlib;
use window_system::WindowSystem;

pub struct KeyPressedHandler; 
pub struct MapRequestHandler;

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

impl KeyPressedHandler {
    pub fn new() -> KeyPressedHandler {
        return KeyPressedHandler;
    }

    pub fn handle(&self) {
        println!("KeyPressed");
    }
}
