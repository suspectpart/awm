use x11::xlib;
use window_system::WindowSystem;
use libc::{c_ulong};

pub struct KeyPressedHandler; 
pub struct MapRequestHandler;

fn create_some_window(window_system: &WindowSystem, width: u32, height: u32, x: i32, y: i32) -> c_ulong {
    let border_width = 2;
    
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

        let height: u32;
        let width: u32;
        let mut x: i32 = 0;
        let y: i32 = 0;

        if window_system.count.get() == 0 {
           width = window_system.info.width as u32;
           height = window_system.info.height as u32;
        }
        else {
            width = (window_system.info.width / 2) as u32;
            height = window_system.info.height as u32;
            x = width as i32;
        }

        // create frame as a new parent for the window to be mapped
        let frame = create_some_window(window_system, width, height, x, y);
        unsafe {
            // resize window to fit parent
            xlib::XResizeWindow(window_system.display, event.window, width as u32, height as u32);

            // make frame window parent of window to be mapped
            xlib::XReparentWindow(window_system.display, event.window, frame, 0, 0);

            // show frame
            xlib::XMapWindow(window_system.display, frame); 

            // show window inside frame
            xlib::XMapWindow(window_system.display, event.window);
        }

        window_system.count.set(window_system.count.get() + 1);
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
