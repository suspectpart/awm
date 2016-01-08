// Copying the event codes here so I can use them in pattern matching
// (in the rust xlib, they are static and can not be used for pattern matching)

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use libc::*;

pub const KeyPress:c_int         = 2;
pub const KeyRelease:c_int       = 3;
pub const ButtonPress:c_int      = 4;
pub const ButtonRelease:c_int    = 5;
pub const MotionNotify:c_int     = 6;
pub const EnterNotify:c_int      = 7;
pub const LeaveNotify:c_int      = 8;
pub const FocusIn:c_int          = 9;
pub const FocusOut:c_int         = 10;
pub const KeymapNotify:c_int     = 11;
pub const Expose:c_int           = 12;
pub const GraphicsExpose:c_int   = 13;
pub const NoExpose:c_int         = 14;
pub const VisibilityNotify:c_int = 15;
pub const CreateNotify:c_int     = 16;
pub const DestroyNotify:c_int    = 17;
pub const UnmapNotify:c_int      = 18;
pub const MapNotify:c_int        = 19;
pub const MapRequest:c_int       = 20;
pub const ReparentNotify:c_int   = 21;
pub const ConfigureNotify:c_int  = 22;
pub const ConfigureRequest:c_int = 23;
pub const GravityNotify:c_int    = 24;
pub const ResizeRequest:c_int    = 25;
pub const CirculateNotify:c_int  = 26;
pub const CirculateRequest:c_int = 27;
pub const PropertyNotify:c_int   = 28;
pub const SelectionClear:c_int   = 29;
pub const SelectionRequest:c_int = 30;
pub const SelectionNotify:c_int  = 31;
pub const ColormapNotify:c_int   = 32;
pub const ClientMessage:c_int    = 33;
pub const MappingNotify:c_int    = 34;
pub const GenericEvent:c_int     = 35;
pub const LASTEvent:c_int        = 36;
