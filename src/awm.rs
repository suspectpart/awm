extern crate libc;
extern crate xlib;

use xlib::*;
use window_system::WindowSystem;

mod window_system;

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
        let window = XCreateSimpleWindow(window_system.display, window_system.root, x,y, width, height,border_width, border,background);
        //how do I use the flags to get an i_64 instead of a struct? ignore for now.
        //XSelectInput(window_system.display, window, ExposureMask | KeyPressMask );
        XSelectInput(window_system.display, window, (1 << 0) | (1 << 15));
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
                println!("Bye Bye");
                break;
            }
        }
    }
}
