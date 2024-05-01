use copy_to_output::copy_to_output;
use std::env;

fn main() {
    if cfg!(target_os = "windows") {
        // Re-runs script if any files in res are changed
        println!("cargo:rerun-if-changed=shawl.exe");
        copy_to_output("shawl.exe", &env::var("PROFILE").unwrap()).expect("Could not copy");
    }
}
