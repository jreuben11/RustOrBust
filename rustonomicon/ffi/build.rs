fn main() {
    // Link against the system snappy library (install with: sudo apt install libsnappy-dev)
    println!("cargo:rustc-link-lib=snappy");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rerun-if-changed=build.rs");
}
