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
      
        // create frame as a new parent for the window to be mapped
        let frame = create_some_window(window_system);
        unsafe {
            // resize window to fit parent
            xlib::XResizeWindow(window_system.display, event.window, 500,300);

            // make frame window parent of window to be mapped
            xlib::XReparentWindow(window_system.display, event.window, frame, 0, 0);

            // show frame
            xlib::XMapWindow(window_system.display, frame); 

            // show window inside frame
            xlib::XMapWindow(window_system.display, event.window);
        }
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
