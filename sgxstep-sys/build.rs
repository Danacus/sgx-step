use std::{env, path::PathBuf};

fn main() {
    cc::Build::new()
        .files(
            glob::glob("../libsgxstep/*.c")
                .unwrap()
                .filter_map(|e| e.ok()),
        )
        .cargo_metadata(true)
        .compile("sgxstep");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}