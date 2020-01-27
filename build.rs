extern crate bindgen;

use std::env;
use std::env::consts;
use std::path::PathBuf;

fn main() {
	println!("cargo:rustc-link-lib=k4a"); 
    println!("cargo:rerun-if-changed=wrapper.h");
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-search=./vendor/lib/linux");
    }
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-search=./vendor/lib/windows/{}", consts::ARCH);
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I./vendor/include/{}", consts::OS))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
