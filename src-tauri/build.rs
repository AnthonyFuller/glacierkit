use std::path::Path;
use std::env;

fn main() {
    // Windows-specific linking
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-search=ResourceLib/ResourceLib-win-x64");
        println!("cargo:rustc-link-lib=ResourceLib_HM2016");
        println!("cargo:rustc-link-lib=ResourceLib_HM2");
        println!("cargo:rustc-link-lib=ResourceLib_HM3");
    }

    // Linux-specific linking
    #[cfg(target_os = "linux")]
    {
		let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
		let resourcelib_dir = Path::new(&dir).join("ResourceLib/ResourceLib-linux-x64");
        
		println!("cargo:rustc-link-search={}", resourcelib_dir.display());        
		println!("cargo:rustc-link-arg=-Wl,-rpath={}", resourcelib_dir.display());

		println!("cargo:rustc-link-lib=dylib:+verbatim=ResourceLib_HM2016.so");
        println!("cargo:rustc-link-lib=dylib:+verbatim=ResourceLib_HM2.so");
        println!("cargo:rustc-link-lib=dylib:+verbatim=ResourceLib_HM3.so");

        println!("cargo:include={}", resourcelib_dir.join("include").display());
    }

    tauri_build::build();
}