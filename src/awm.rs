extern crate libc;
extern crate x11;

mod handlers;
mod window_system;
mod events;

use window_system::WindowSystem;
use x11::xlib;


fn main() {
    let window_system = WindowSystem::new();

    let substructure_notify_mask = 0x1 << 19;
    let substructure_redirect_mask = 0x1 << 20;

    unsafe {
        let border = xlib::XWhitePixel(window_system.display, xlib::XDefaultScreen(window_system.display));
        let background = xlib::XBlackPixel(window_system.display, xlib::XDefaultScreen(window_system.display));
        let x = 50;
        let y = 50;
        let width = 100;
        let height = 100;
        let border_width = 10;
        let window = xlib::XCreateSimpleWindow(window_system.display, 
                                         window_system.root, 
                                         x, 
                                         y, 
                                         width, 
                                         height,
                                         border_width, 
                                         border,background);

        xlib::XSelectInput(window_system.display, 
                     window_system.root, 
                     substructure_notify_mask  | substructure_redirect_mask);
        
        xlib::XMapWindow(window_system.display, window);
    }

    let mut e = xlib::XEvent { pad: [0;24] };
    
    loop {
        unsafe {
            xlib::XNextEvent(window_system.display, &mut e);
            let test: xlib::XMapRequestEvent = xlib::XMapRequestEvent::from(e);
            match e.get_type() {
                events::Expose => println!("Expose"),
                events::KeyPress => println!("KeyPress"),
                events::MapRequest => handlers::MapRequestHandler::new().handle(test, &window_system),
                events::CreateNotify => println!("CreateNotify"),
                events::ReparentNotify => println!("Reparent"),
                _ => println!("Unknown Event"),
            }
        }
    }
}
