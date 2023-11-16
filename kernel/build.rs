#[cfg(target_arch = "x86_64")]
fn main() {
    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-Tlinker-x86_64.ld");
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed=linker-x86_64.ld");
}

#[cfg(target_arch = "aarch64")]
fn main() {
    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-Tlinker-aarch64.ld");
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed=linker-aarch64.ld");
}

#[cfg(target_arch = "riscv64")]
fn main() {
    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-Tlinker-riscv64.ld");
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed=linker-riscv64.ld");
}