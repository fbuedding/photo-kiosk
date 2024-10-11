use std::{env, process::Command};

fn main() {
    //=10.9

    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("make")
        .arg("PLATFORM=PLATFORM_DESKTOP")
        .current_dir("raylib/src")
        .status()
        .unwrap();
    Command::new("cp")
        .args(["libraylib.a", &out_dir])
        .current_dir("raylib/src")
        .status()
        .unwrap();

    println!("cargo::rustc-link-search={}", out_dir);
    println!("cargo::rustc-link-lib=static=raylib");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        println!("cargo::rustc-link-lib=framework=CoreVideo");
        println!("cargo::rustc-link-lib=framework=IOKit");
        println!("cargo::rustc-link-lib=framework=Cocoa");
        println!("cargo::rustc-link-lib=framework=GLUT");
        println!("cargo::rustc-link-lib=framework=OpenGL");
    }
}
