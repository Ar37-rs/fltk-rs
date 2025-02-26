use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub fn get(target_os: &str, out_dir: &Path) {
    let pkg_version = env::var("CARGO_PKG_VERSION").unwrap();
    if let Ok(cfltk_path) = env::var("CFLTK_BUNDLE_DIR") {
        println!("cargo:rustc-link-search=native={}", cfltk_path);
    } else {
        let url = if let Ok(cfltk_url) = env::var("CFLTK_BUNDLE_URL") {
            PathBuf::from(cfltk_url)
        } else {
            let mut platform = target_os.to_string();

            if target_os == "windows" {
                if cfg!(target_env = "gnu") {
                    platform.push_str("-gnu");
                } else {
                    platform.push_str("-msvc");
                }
            }

            PathBuf::from(format!(
                "{}/{}/lib_x64-{}.tar.gz",
                env::var("CFLTK_BUNDLE_URL_PREFIX").unwrap_or_else(|_| String::from(
                    "https://github.com/fltk-rs/fltk-rs/releases/download"
                )),
                pkg_version,
                platform
            ))
        };

        let curl_status = Command::new("curl")
            .args(&["-LOkf", url.to_str().unwrap()])
            .current_dir(out_dir)
            .status()
            .expect("Curl is needed to download the bundled libraries!");

        if !curl_status.success() {
            panic!("Download bundled libraries from {:?} failed", url)
        }

        let tar_status = Command::new("tar")
            .args(&["-xzvf", url.file_name().unwrap().to_str().unwrap()])
            .current_dir(out_dir)
            .status()
            .expect("Tar is needed to upack the bundled libraries!");

        if !tar_status.success() {
            panic!("Unpack bundled libraries failed")
        }
    }
}
