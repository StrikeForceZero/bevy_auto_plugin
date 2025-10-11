use rustc_version::{Channel, version_meta};

fn main() {
    println!("cargo:rustc-check-cfg=cfg(nightly)");
    println!("cargo:rustc-check-cfg=cfg(stable)");
    println!("cargo:rustc-check-cfg=cfg(wasm)");
    // println!("cargo:rustc-check-cfg=cfg(web)");
    // println!("cargo:rustc-check-cfg=cfg(wasi)");

    let version = version_meta().expect("Failed to get rustc version");
    if version.channel == Channel::Nightly {
        println!("cargo:rustc-cfg=nightly");
    } else {
        println!("cargo:rustc-cfg=stable");
    }
    cfg_aliases::cfg_aliases! {
        // Any WebAssembly target
        wasm: { target_arch = "wasm32" },
        // unlikely needed, but left for preferred naming conventions
        // Browser WASM (using bevy’s "web" convention)
        // web: { all(target_arch = "wasm32", not(target_os = "wasi")) },
        // WASI runtime
        // wasi: { all(target_arch = "wasm32", target_os = "wasi") },
    }
}
