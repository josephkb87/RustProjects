// Example custom [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=src/main.rs");
}
