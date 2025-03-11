use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=FFMPEG_ROOT");

    let (include_paths, lib_paths) = if let Some(ffmpeg_root) = env::var("FFMPEG_ROOT").ok() {
        let ffmpeg_root = PathBuf::from(ffmpeg_root);
        (
            vec![ffmpeg_root.join("include")],
            vec![ffmpeg_root.join("lib")],
        )
    } else {
        let lib = pkg_config::probe_library("libavcodec")
            .expect("could not find libavcodec using pkg-config, please set FFMPEG_ROOT env var");
        (lib.include_paths, lib.link_paths)
    };

    for lib_path in lib_paths {
        println!("cargo:rustc-link-search={}", lib_path.display());
    }

    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=swresample");
    println!("cargo:rustc-link-lib=swscale");

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_type("AVCodec")
        .allowlist_type("AVCodecContext")
        .allowlist_type("AVPacket")
        .allowlist_type("AVFrame")
        .allowlist_type("AVStream")
        .allowlist_type("AVFormatContext")
        .allowlist_type("AVCodecParserContext")
        .allowlist_type("AVCodecParameters")
        .allowlist_type("AVRational")
        .allowlist_type("AVDictionary")
        .allowlist_type("AVOption")
        .allowlist_type("SwsContext")
        .allowlist_type("SwsFilter")
        .allowlist_type("SwrContext")
        .allowlist_item("SWS_.*")
        .allowlist_item("AVERROR_.*")
        .allowlist_item("AVError.*")
        .allowlist_function("av_.*")
        .allowlist_function("sws_.*")
        .allowlist_function("swr_.*")
        .allowlist_function("avformat_.*")
        .allowlist_function("avcodec_.*")
        .allowlist_function("avutil_.*")
        .allowlist_function("avfilter_.*")
        .allowlist_function("avdevice_.*")
        .allowlist_function("avresample_*");

    for include_path in include_paths {
        builder = builder.clang_arg(format!("-I{}", include_path.display()));
    }

    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write libavcodec bindings");
}
