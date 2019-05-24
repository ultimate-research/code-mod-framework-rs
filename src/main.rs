#![allow(dead_code)]
#![feature(lang_items, linkage, start, no_core)]
#![no_core]

//extern crate libnx_rs;
//extern crate libc;

/*use std::result::Result;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io;
use std::os::unix::io::AsRawFd;
*/
mod saltynx;

#[lang="sized"]
trait Sized {}

#[lang="copy"]
trait Copy {}

#[lang="freeze"]
trait Freeze {}

//#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! {loop{}}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[start]
pub fn main(a: isize, b: *const *const u8) -> isize {
    /*if let Some(ver_ptr) = saltynx::find_code(b"Ver. %d.%d.%d") {
        match saltynx::memcpy(ver_ptr, b"noice v%d%d%d") {
            Ok(_) => {}
            Err(_) => { loop {} }
        }
    } else {
        loop {}
    }*/
    unsafe {
        const ver_string: &'static [u8; 13] = b"Ver. %d.%d.%d";
        const new_ver: &'static [u8; 13] = b"noice v%d%d%d";
        let ver_ptr = saltynx::S_findCode(
            ver_string as *const u8,
            13
        );
        saltynx::S_Memcpy(
            ver_ptr as _,
            new_ver as *const u8 as _,
            13
        );
    }
    0
}
/*
pub fn redirect_stdout (filename : &str) -> Result<File, io::Error> {
    let mut outfile = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)?;
    outfile.write_fmt(format_args!("Redirecting standard output to {}.", filename))?;
    let raw_fd = outfile.as_raw_fd();
    let new_fd = unsafe {
        libc::fflush(0 as *mut libc::FILE);
        libc::dup2(raw_fd, libc::STDOUT_FILENO)
    };
    if new_fd != libc::STDOUT_FILENO {
        Err(io::Error::new(io::ErrorKind::Other, format!("Could not call dup2. Ended up redirecting fd {} to {} instead of {}.", raw_fd, new_fd, libc::STDOUT_FILENO)))
    }
    else { 
        Ok(outfile) 
    }
}

pub fn redirect_stderr (filename : &str) -> Result<File, io::Error> {
    let mut outfile = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)?;
    outfile.write_fmt(format_args!("Redirecting standard error to {}.\n", filename))?;
    let raw_fd = outfile.as_raw_fd();
    let new_fd = unsafe {
        libc::fflush(0 as *mut libc::FILE);
        libc::dup2(raw_fd, libc::STDERR_FILENO)
    };
    if new_fd != libc::STDERR_FILENO {
        Err(io::Error::new(io::ErrorKind::Other, format!("Could not call dup2. Ended up redirecting fd {} to {} instead of {}.", raw_fd, new_fd, libc::STDERR_FILENO)))
    }
    else { 
        Ok(outfile) 
    }
}
*/
