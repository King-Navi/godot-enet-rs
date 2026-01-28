/**
 * You will need the /thirdparty/enet directory from the godot engine
 */

use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("TARGET").unwrap();
    let mut build = cc::Build::new();
    build
        .include("thirdparty/enet")
        .include("thirdparty/enet/enet")
        .file("thirdparty/enet/callbacks.c")
        .file("thirdparty/enet/compress.c")
        .file("thirdparty/enet/host.c")
        .file("thirdparty/enet/list.c")
        .file("thirdparty/enet/packet.c")
        .file("thirdparty/enet/peer.c")
        .file("thirdparty/enet/protocol.c");

    if target_os == "windows" {
        //build.file("thirdparty/enet/win32.c");
        //build.define("WIN32", None);
        panic!("No implemented");
    } else {
        build.file("thirdparty/enet/unix.c");
        build.define("HAS_FCNTL", None);
        build.define("HAS_POLL", None);
        build.define("HAS_GETADDRINFO", None);
        build.define("HAS_GETNAMEINFO", None);
        build.define("HAS_INET_PTON", None);
        build.define("HAS_INET_NTOP", None);
    }

    build.compile("godot-enet");

    let bindings = bindgen::Builder::default()
        .header("thirdparty/enet/enet/enet.h")
        .clang_arg("-Ithirdparty/enet") 
        .clang_arg(format!("--target={}", target_arch))
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("No se pudieron generar los bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("No se pudo escribir el archivo de bindings");

    println!("cargo:rerun-if-changed=thirdparty/enet");
    println!("cargo:rerun-if-changed=build.rs");
}