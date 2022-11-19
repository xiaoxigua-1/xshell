use libc::passwd;
use std::ffi::CStr;
use std::ptr::read;

pub fn whoami() -> &'static str {
    unsafe {
        let pwd_pointer: *mut passwd = libc::getpwuid(libc::geteuid());
        let pwd = read(pwd_pointer);
        CStr::from_ptr(pwd.pw_name).to_str().unwrap()
    }
}

