extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn whitelist_from_file<F>(mut builder: bindgen::Builder, name: &str, mut f: F) -> bindgen::Builder
where
    F: FnMut(bindgen::Builder, String) -> bindgen::Builder,
{
    let infile: PathBuf = env::var_os("CARGO_MANIFEST_DIR").unwrap().into();
    let infile = File::open(infile.join(name)).expect("Failed to open whitelist file");

    for line in BufReader::new(infile).lines() {
        let mut line: String = line.expect("whitelist IO error");
        if let Some(idx) = line.find('#') {
            line.truncate(idx);
        }

        if line.is_empty() {
            continue;
        }
        builder = f(builder, line);
    }

    builder
}

fn main() {
    println!("{}", std::env::var("TARGET").unwrap());

    if !std::env::var("TARGET").unwrap().contains("-unix") {
        return;
    }

    println!("cargo:rustc-link-lib=avahi-client");
    println!("cargo:rustc-link-lib=avahi-common");

    let mut bindings = bindgen::Builder::default()
        .whitelist_recursively(false)
        .ctypes_prefix("::libc")
        .size_t_is_usize(true)
        .header("wrapper.h")
        .bitfield_enum("AvahiClientFlags");

    bindings = whitelist_from_file(bindings, "types.txt", bindgen::Builder::whitelisted_type);
    bindings = whitelist_from_file(
        bindings,
        "functions.txt",
        bindgen::Builder::whitelisted_function,
    );

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
