extern crate libc;

use libc::{c_int, c_char, size_t};

extern {
    pub fn getzoneid() -> c_int;
    pub fn getzoneidbyname(name: *const c_char) -> c_int;
    pub fn getzonenamebyid(id: c_int, buf: *mut c_char, len: size_t) -> c_int;
}
