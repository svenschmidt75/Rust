fn main() {
    // This tells Cargo to pass the -ObjC flag to the linker
    // only when we are building on macOS.
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos" {
        println!("cargo:rustc-link-arg=-ObjC");
    }
}