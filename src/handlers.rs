use x11::xlib;
use window_system::WindowSystem;
use libc::c_ulong;

pub struct KeyPressedHandler; 
pub struct MapRequestHandler;

fn create_some_window(window_system: &WindowSystem) -> c_ulong {
    let x = 0;
    let y = 0;
    let width = 500;
    let height = 300;
    let border_width=5;
    unsafe {
        let border = xlib::XWhitePixel(window_system.display, 
                                       xlib::XDefaultScreen(window_system.display));
        let background = xlib::XBlackPixel(window_system.display, 
                                           xlib::XDefaultScreen(window_system.display));
        let window = xlib::XCreateSimpleWindow(window_system.display, 
                                     window_system.root, 
                                     x, 
                                     y, 
                                     width, 
                                     height,
                                     border_width, 
                                     border,background);
        xlib::XSelectInput(window_system.display, 
                           window, 
                           xlib::SubstructureNotifyMask | xlib::SubstructureRedirectMask);

        return window;
    }
}

impl MapRequestHandler {
    pub fn new() -> MapRequestHandler {
        return MapRequestHandler;
    }

    pub fn handle(&self, event: xlib::XEvent, window_system: &WindowSystem) {
        let event = xlib::XMapRequestEvent::from(event);
       
        let frame = create_some_window(window_system);
        unsafe {
            xlib::XResizeWindow(window_system.display, event.window, 500,300);
            xlib::XReparentWindow(window_system.display, event.window, frame, 0, 0);
            xlib::XMapWindow(window_system.display, frame); 
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
