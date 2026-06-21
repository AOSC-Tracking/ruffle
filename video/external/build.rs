fn main() {
    println!("cargo::rustc-link-lib=openh264");
    bindgen::Builder::default()
            .header("/usr/include/wels/codec_api.h")
            .prepend_enum_name(false)
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file("./src/decoder/openh264_sys.rs")
            .expect("Failed to write bindings to file");
}
