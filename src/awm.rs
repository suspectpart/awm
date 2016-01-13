#![allow(non_upper_case_globals)]

extern crate libc;
extern crate x11;

mod handlers;
mod window_system;

use window_system::WindowSystem;
use x11::xlib::*;


fn main() {
    let window_system = WindowSystem::new();

    unsafe {
        let border = XWhitePixel(window_system.display, XDefaultScreen(window_system.display));
        let background = XBlackPixel(window_system.display, XDefaultScreen(window_system.display));
        let x = 50;
        let y = 50;
        let width = 100;
        let height = 100;
        let border_width = 10;
        let window = XCreateSimpleWindow(window_system.display, 
                                         window_system.root, 
                                         x, 
                                         y, 
                                         width, 
                                         height,
                                         border_width, 
                                         border,background);

        XSelectInput(window_system.display, 
                     window_system.root, 
                         SubstructureNotifyMask | SubstructureRedirectMask);
        
        XMapWindow(window_system.display, window);
    }

    let mut e = XEvent { pad: [0;24] };
    
    loop {
        unsafe {
            XNextEvent(window_system.display, &mut e);
            let test: XMapRequestEvent = XMapRequestEvent::from(e);
            match e.get_type() {
                Expose => println!("Expose"),
                KeyPress => handlers::KeyPressedHandler::new().handle(),
                MapRequest => handlers::MapRequestHandler::new().handle(test, &window_system),
                CreateNotify => println!("CreateNotify"),
                ReparentNotify => println!("Reparent"),
                _ => println!("Unknown Event"),
            }
        }
    }
}
