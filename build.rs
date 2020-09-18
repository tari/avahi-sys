extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    if !std::env::var("TARGET").unwrap().contains("-linux") {
        return;
    }

    println!("cargo:rustc-link-lib=avahi-client");
    println!("cargo:rustc-link-lib=avahi-common");

    bindgen::Builder::default()
        .header("wrapper.h")
        .ctypes_prefix("::libc")
        .size_t_is_usize(true)
        .bitfield_enum("AvahiClientFlags")
        .generate()
        .expect("failed to generate bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("failed to write bindings to file");
}
