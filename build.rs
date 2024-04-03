use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=external/jigsaw/src/jigsaw.cpp");
    println!("cargo:rustc-link-lib=static=jigsaw");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-search=native={}", out_path.display());
    cc::Build::new()
        .cpp(true)
        .file("external/jigsaw/src/jigsaw.cpp")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-pedantic")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .flag_if_supported("-Wshadow")
        .flag_if_supported("-Wfloat-conversion")
        .flag_if_supported("-fno-math-errno")
        .flag_if_supported("-fno-trapping-math")
        .flag_if_supported("-ffinite-math-only")
        .flag_if_supported("-fopenmp")
        .flag_if_supported("-flto")
        .flag_if_supported("-O3")
        .flag_if_supported("-DNDEBUG")
        .flag_if_supported("-DLIB_JIGSAW")
        .include("external/jigsaw/inc")
        .compile("libjigsaw.a");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
