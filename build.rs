use std::{env, process::Command};

fn main() {
    //=10.9

    let target = env::var("TARGET").unwrap();

    Command::new("make")
        .args(["clean"])
        .current_dir("raylib/src")
        .status()
        .unwrap();
    match target.as_str() {
        "armv7-unknown-linux-gnueabihf" => {
            Command::new("make")
                .args(["PLATFORM=PLATFORM_DRM"])
                .current_dir("raylib/src")
                .status()
                .unwrap();

            println!("cargo::rustc-link-search=/usr/local/include");
            println!("cargo::rustc-link-search=/usr/include/libdrm");
            println!("cargo::rustc-link-search=./raylib/src");
            println!("cargo::rustc-link-search=./raylib/src/external");
            println!("cargo::rustc-link-search=./raylib/src/external/glfw/include");
            println!("cargo::rustc-link-lib=raylib");
            println!("cargo::rustc-link-lib=GLESv2");
            println!("cargo::rustc-link-lib=EGL");
            println!("cargo::rustc-link-lib=pthread");
            println!("cargo::rustc-link-lib=rt");
            println!("cargo::rustc-link-lib=m");
            println!("cargo::rustc-link-lib=gbm");
            println!("cargo::rustc-link-lib=drm");
            println!("cargo::rustc-link-lib=dl");
            println!("cargo::rustc-link-lib=atomic");
            println!("cargo::rustc-link-lib=static=raylib");
        }
        "aarch64-apple-darwin" => {
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

            println!("cargo::rustc-link-lib=framework=CoreVideo");
            println!("cargo::rustc-link-lib=framework=IOKit");
            println!("cargo::rustc-link-lib=framework=Cocoa");
            println!("cargo::rustc-link-lib=framework=GLUT");
            println!("cargo::rustc-link-lib=framework=OpenGL");
            println!("cargo::rustc-link-search={}", out_dir);
            println!("cargo::rustc-link-lib=static=raylib");
        }
        _ => println!("cargo:warning=Target not supported{}", target),
    }
    Command::new("rm")
        .args(["libraylib.a"])
        .current_dir("raylib/src")
        .status()
        .unwrap();
}
