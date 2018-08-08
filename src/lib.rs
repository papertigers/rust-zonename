extern crate errno;
extern crate libc;

use std::ffi::CString;
use libc::{c_char,size_t};
use errno::{Errno,errno};

mod ffi;

pub const ZONENAME_MAX: usize = 64;

pub fn getzoneid() -> Result<i32, Errno> {
    let zoneid = unsafe {
        ffi::getzoneid()
    };

    match zoneid {
        -1 => Err(errno()),
        _ => Ok(zoneid),
    }
}

pub fn getzoneidbyname(zonename: &str) -> Result<i32, Errno> {
    let c_zonename = CString::new(zonename).expect("Failed to read zonename");

    let zoneid = unsafe {
        ffi::getzoneidbyname(c_zonename.as_ptr())
    };

    match zoneid {
        -1 => Err(errno()),
        _ => Ok(zoneid),
    }
}

pub fn getzonenamebyid(id: i32) -> Result<String, Errno> {
    let mut buf: Vec<c_char> = Vec::with_capacity(ZONENAME_MAX);
    let ptr = buf.as_mut_ptr() as *mut libc::c_char;

    let len = unsafe {
        ffi::getzonenamebyid(id, ptr, buf.capacity() as size_t)
    };
    if len < 0 {
        return Err(errno());
    }

    assert!(len >= 0);

    let cstring = unsafe {
        CString::from_raw(ptr)
    };

    Ok(cstring.into_string().expect("Failed to read zonename"))
}

pub fn getzonename() -> Result<String, Errno> {
    let id = getzoneid()?;
    getzonenamebyid(id)
}
