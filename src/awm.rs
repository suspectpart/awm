extern crate libc;
extern crate xlib;
extern crate xinerama;

/*
use xlib::XCreateSimpleWindow;
use xlib::XBlackPixel;
use xlib::XWhitePixel;
use xlib::XMapWindow;
use xlib::XDefaultScreen;
use xlib::XEvent;
use xlib::XNextEvent;
use xlib::XSelectInput;
use xlib::XInputEventMasks;
*/
use xlib::*;
use window_system::WindowSystem;


mod window_system;

fn main() {
	println!("Hallo, Welt!");

    let window_system = WindowSystem::new();
    unsafe {
        let border = XBlackPixel(window_system.display, XDefaultScreen(window_system.display));
        let background = XWhitePixel(window_system.display, XDefaultScreen(window_system.display));
        let x = 50;
        let y = 50;
        let width = 100;
        let height = 100;
        let border_width = 10;
        let window = XCreateSimpleWindow(window_system.display, window_system.root, x,y, width, height,border_width, border,background);
        //XSelectInput(window_system.display, window, ExposureMask | KeyPressMask );
        XSelectInput(window_system.display, window, (1 << 0) | (1 << 15));
        XMapWindow(window_system.display, window);
    }

    let mut e = XEvent { _type : 0, pad: [0;24] };
    
    while true {
        unsafe {
            XNextEvent(window_system.display, &mut e);
            if(e._type == Expose) {println!("test");}
            if(e._type == KeyPress) {break;}
        }

    }
}
