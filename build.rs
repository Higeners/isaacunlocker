use std::env;
fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        use std::{
            fs::{copy, write},
            path::PathBuf,
            process::Command,
        };
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        copy(manifest_dir.join("icon.ico"), out_dir.join("icon.ico")).unwrap();
        write(out_dir.join("icon.rc"), "icon ICON icon.ico").unwrap();
        Command::new("windres")
            .current_dir(&out_dir)
            .arg("icon.rc")
            .arg("icon.lib")
            .spawn()
            .unwrap();
        println!(
            "cargo:rustc-link-search={}",
            out_dir.into_os_string().into_string().unwrap()
        );
        println!("cargo:rustc-link-lib=icon");
    }
}