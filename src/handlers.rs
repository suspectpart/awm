use x11::xlib;
use window_system::WindowSystem;

pub struct KeyPressedHandler; 
pub struct MapRequestHandler;

impl MapRequestHandler {
    pub fn new() -> MapRequestHandler {
        return MapRequestHandler;
    }

    pub fn handle(&self, event: xlib::XEvent, window_system: &WindowSystem) {
        let event = xlib::XMapRequestEvent::from(event);
        
        unsafe {
            xlib::XMapWindow(window_system.display, event.window); 
        }

        println!("Handling MapRequest");
    }
}

impl KeyPressedHandler {
    pub fn new() -> KeyPressedHandler {
        return KeyPressedHandler;
    }

    pub fn handle(&self, event: xlib::XEvent) {
        let event = xlib::XKeyPressedEvent::from(event);
        println!("KeyPressed {}", event.keycode);
    }
}
