use copy_to_output::copy_to_output;
use std::env;
#[cfg(target_os = "windows")]
fn elevate_privileges() {
    use std::{io::Write, process};
    // only build the resource for release builds
    // as calling rc.exe might be slow
    let mut res = winres::WindowsResource::new();
    res //.set_icon("resources\\ico\\fiscalidade_server.ico")
        .set_manifest(
            r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#,
        );
    if let Err(error) = res.compile() {
        eprint!("{}", error);
        process::exit(1);
    }
}

#[cfg(target_os = "windows")]
fn main() {
    elevate_privileges();
    println!("cargo:rerun-if-changed=shawl.exe");
    copy_to_output("shawl.exe", &env::var("PROFILE").unwrap()).expect("Could not copy");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
