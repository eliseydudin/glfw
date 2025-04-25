use std::path::PathBuf;

fn main() {
    let glfw = pkg_config::Config::new()
        .probe("glfw3")
        .expect("GLFW couldn't be located using pkg-config");

    glfw.libs
        .iter()
        .for_each(|lib| println!("cargo::rustc-link-lib={lib}"));
    glfw.link_paths
        .iter()
        .for_each(|path| println!("cargo::rustc-link-search={}", path.to_string_lossy()));

    let builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args({
            glfw.include_paths
                .iter()
                .map(|s| s.to_str().expect("Should always be a valid string"))
                .map(|s| "-I".to_owned() + s)
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
