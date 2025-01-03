mod host_specific {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    include!("build_linux.rs");   

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    pub fn build() {
        // Print a emulation_mode to silence clippy's unexpected cfg on macOS
        println!("cargo:rustc-cfg=emulation_mode=\"usermode\"");
        println!("cargo:warning=libafl_qemu only builds on Linux hosts");
    }
}

#[rustversion::nightly]
fn nightly() {
    println!("cargo:rustc-cfg=nightly");
}

#[rustversion::not(nightly)]
fn nightly() {}

fn main() {
    println!("cargo:rustc-check-cfg=cfg(nightly)");
    nightly();
    host_specific::build();
}
