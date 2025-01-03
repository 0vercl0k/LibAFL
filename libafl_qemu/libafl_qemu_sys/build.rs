#![forbid(unexpected_cfgs)]

mod host_specific {
    // XXX:
    // cargo c --target=x86_64-pc-windows-gnu
    // https://wiki.qemu.org/Hosts/W32#Native_builds_with_MSYS2
    // pacman -S mingw-w64-x86_64-ninja git
    // enable long paths
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    include!("build_linux.rs");

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    pub fn build() {
        println!("cargo:warning=libafl_qemu_sys only builds on Linux & Windows hosts ATM");
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
    println!(r#"cargo::rustc-check-cfg=cfg(emulation_mode, values("usermode", "systemmode"))"#);
    println!(
        r#"cargo::rustc-check-cfg=cfg(cpu_target, values("arm", "aarch64", "hexagon", "i386", "mips", "ppc", "x86_64"))"#
    );
    nightly();
    host_specific::build();
}
