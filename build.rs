#![allow( non_upper_case_globals
        , non_camel_case_types
        , non_snake_case
        , dead_code)]

extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main(){

  linklibs();
  generate_bindings();

}

fn linklibs(){
  let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  println!("cargo:rustc-link-search={}/mybind",project_dir);
  println!("cargo:rustc-link-lib=mybind");
  println!("■■■■■■■■■■■■■");
}

fn generate_bindings(){
  let bindings = bindgen::Builder::default()
        .clang_arg("-I./mybind/include")
        .clang_arg("-I./mybind/include2")
        .header("mybind.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
