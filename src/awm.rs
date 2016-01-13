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
        XSelectInput(window_system.display, 
                     window_system.root, 
                         SubstructureNotifyMask | SubstructureRedirectMask);
    }

    let mut event = XEvent { pad: [0;24] };
    
    loop {
        unsafe {
            XNextEvent(window_system.display, &mut event);

            match event.get_type() {
                Expose => println!("Expose"),
                KeyPress => handlers::KeyPressedHandler::new().handle(event),
                MapRequest => handlers::MapRequestHandler::new().handle(event, &window_system),
                CreateNotify => println!("CreateNotify"),
                ReparentNotify => println!("Reparent"),
                _ => println!("Unknown Event"),
            }
        }
    }
}
