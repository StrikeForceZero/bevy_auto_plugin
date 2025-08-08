use rustc_version::{Channel, version_meta};

fn main() {
    println!("cargo:rustc-check-cfg=cfg(nightly)");
    println!("cargo:rustc-check-cfg=cfg(stable)");

    let version = version_meta().expect("Failed to get rustc version");
    if version.channel == Channel::Nightly {
        println!("cargo:rustc-cfg=nightly");
    } else {
        println!("cargo:rustc-cfg=stable");
    }
    cfg_aliases::cfg_aliases! {
        wasm: { target_arch = "wasm32" },
    }
}
