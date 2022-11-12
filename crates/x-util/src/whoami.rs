#[cfg(target_family = "unix")]
use libc::passwd;

#[cfg(target_family = "unix")]
use std::ffi::CStr;

#[cfg(target_family = "unix")]
use std::ptr::read;

#[cfg(target_family = "windows")]
use winsafe::GetUserName;

#[cfg(target_family = "unix")]
fn get_username() -> &'static str {
    unsafe {
        let pwd_pointer: *mut passwd = libc::getpwuid(libc::geteuid());
        let pwd = read(pwd_pointer);
        CStr::from_ptr(pwd.pw_name).to_str().unwrap()
    }
}

#[cfg(target_family = "windows")]
fn get_username() -> String {
    GetUserName().unwrap()
}

pub fn whoami() -> &'static str {
    get_username()
}

#[test]
fn test_whoami() {
    assert!(!whoami().is_empty());
    println!("{}", whoami());
}
