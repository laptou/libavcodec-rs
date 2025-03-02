use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=FFMPEG_ROOT");
    let ffmpeg_root = PathBuf::from(env::var("FFMPEG_ROOT").unwrap());
    let ffmpeg_include = ffmpeg_root.join("include");
    let ffmpeg_lib = ffmpeg_root.join("lib");
    let ffmpeg_bin = ffmpeg_root.join("bin");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", ffmpeg_lib.display());
    println!("cargo:rustc-link-search={}", ffmpeg_bin.display());

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=avcodec-61");
    println!("cargo:rustc-link-lib=avutil-59");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_type("AVCodec*")
        .allowlist_type("AVCodecContext*")
        .allowlist_type("AVPacket*")
        .allowlist_type("AVFrame*")
        .allowlist_type("AVStream*")
        .allowlist_type("AVFormatContext*")
        .allowlist_type("AVCodecParserContext*")
        .allowlist_type("AVCodecParameters*")
        .allowlist_type("AVRational*")
        .allowlist_type("AVDictionary*")
        .allowlist_type("AVOption*")
        .allowlist_function("av_*")
        .allowlist_function("avformat_*")
        .allowlist_function("avcodec_*")
        .allowlist_function("avutil_*")
        .allowlist_function("avfilter_*")
        .allowlist_function("avdevice_*")
        .allowlist_function("avresample_*")
        .clang_arg(format!("-I{}", ffmpeg_include.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write libavcodec bindings");
}
