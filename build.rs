fn main() {
    // Tell the linker to use the memory layout from cortex-m-rt
    println!("cargo:rustc-link-arg-bins=-Tlink.x");

    // Tell the linker to include the defmt logging memory layout
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    // Disable magic section headers (required for bare-metal ARM)
    println!("cargo:rustc-link-arg-bins=--nmagic");
}
