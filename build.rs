// build.rs

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    // aarch64-none-elf-g++ -o saltysd.o saltysd.c -c -I$(DEVKITPRO)/libnx/include
    // aarch64-none-elf-ar rcs saltysd.a saltysd.o 
    Command::new("make")
        .current_dir("./lib")
        .status()
        .expect("SaltySD C bindings failed to compile");
    /*Command::new("gcc").args(&["src/hello.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/hello.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libhello.a", "hello.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();
    */
    //println!("cargo:warning={}", out_dir);
    //println!("cargo:rustc-link-search={}", out_dir);
    //println!("cargo:rustc-link-lib=static=saltysd");
}
