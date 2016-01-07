extern crate libc;
extern crate xlib;

use window_system::WindowSystem;
use xlib::*;

mod window_system;

fn main() {
    let window_system = WindowSystem::new();

    let substructure_notify_mask = 0x1 << 19;
    let substructure_redirect_mask = 0x1 << 20;

    unsafe {
        let border = XWhitePixel(window_system.display, XDefaultScreen(window_system.display));
        let background = XBlackPixel(window_system.display, XDefaultScreen(window_system.display));
        let x = 50;
        let y = 50;
        let width = 100;
        let height = 100;
        let border_width = 10;
        let window = XCreateSimpleWindow(window_system.display, window_system.root, x,y, width, height,border_width, border,background);
        XSelectInput(window_system.display, window_system.root, substructure_notify_mask  | substructure_redirect_mask);
        XMapWindow(window_system.display, window);
    }

    let mut e = XEvent { _type : 0, pad: [0;24] };
    
    loop {
        unsafe {
            XNextEvent(window_system.display, &mut e);

            if e._type == Expose {
                println!("something got exposed");
            }
            if e._type == KeyPress {
                println!("Key Pressed");
            }
            if e._type == CreateNotify {
                println!("CreateNotify");
            }
            if e._type == MapRequest {
                println!("Map Request");
            }
            if e._type == ReparentNotify {
                println!("Reparent");
            }
        }
    }
}
