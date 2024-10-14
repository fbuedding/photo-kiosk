use std::{env, path::Path, process::Command};

fn main() {
    //=10.9

    let target = env::var("TARGET").unwrap();

    let binding = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&binding);
    Command::new("cp")
        .args(["-r", "raylib", out_dir.join("raylib").to_str().unwrap()])
        .status()
        .unwrap();

    match target.as_str() {
        "armv7-unknown-linux-gnueabihf" => {
            Command::new("make")
                .args(["PLATFORM=PLATFORM_DRM"])
                .current_dir(&format!("{}", out_dir.join("raylib/src").to_str().unwrap()))
                .status()
                .unwrap();

            println!("cargo::rustc-link-search=/usr/local/include");
            println!("cargo::rustc-link-search=/usr/include/libdrm");
            println!(
                "cargo::rustc-link-search={}",
                out_dir.join("raylib/src").to_str().unwrap()
            );
            println!("cargo::rustc-link-lib=GLESv2");
            println!("cargo::rustc-link-lib=EGL");
            println!("cargo::rustc-link-lib=pthread");
            println!("cargo::rustc-link-lib=rt");
            println!("cargo::rustc-link-lib=m");
            println!("cargo::rustc-link-lib=gbm");
            println!("cargo::rustc-link-lib=drm");
            println!("cargo::rustc-link-lib=dl");
            println!("cargo::rustc-link-lib=atomic");
        }
        "aarch64-apple-darwin" => {
            Command::new("make")
                .arg("PLATFORM=PLATFORM_DESKTOP")
                .current_dir(&format!("{}", out_dir.join("raylib/src").to_str().unwrap()))
                .status()
                .unwrap();

            println!(
                "cargo::rustc-link-search={}",
                out_dir.join("raylib/src").to_str().unwrap()
            );
            println!("cargo::rustc-link-lib=framework=CoreVideo");
            println!("cargo::rustc-link-lib=framework=IOKit");
            println!("cargo::rustc-link-lib=framework=Cocoa");
            println!("cargo::rustc-link-lib=framework=GLUT");
            println!("cargo::rustc-link-lib=framework=OpenGL");
        }
        _ => println!("cargo:warning=Target not supported:{}", target),
    }
}
