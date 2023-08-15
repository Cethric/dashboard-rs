use std::io::Write;

fn main() {
    println!("cargo:info=begin generating leptos resources");
    let root_dir: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let generated_dir = root_dir.join("generated");
    let js_dir = generated_dir.join("js");

    std::fs::create_dir_all(js_dir.clone()).unwrap();
    std::fs::File::create(js_dir.join("tiptap-bundle.min.js"))
        .unwrap()
        .write_all(leptos_tiptap_build::TIPTAP_BUNDLE_MIN_JS.as_bytes())
        .unwrap();

    std::fs::File::create(js_dir.join("tiptap.js"))
        .unwrap()
        .write_all(leptos_tiptap_build::TIPTAP_JS.as_bytes())
        .unwrap();
    println!("cargo:info=end generating leptos resources");
}