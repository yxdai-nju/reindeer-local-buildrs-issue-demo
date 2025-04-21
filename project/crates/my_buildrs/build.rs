use std::path::Path;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(build_script_run)");
    println!("cargo::rustc-check-cfg=cfg(resource_exists, values(\"true\"))");
    println!("cargo::rustc-check-cfg=cfg(resource_exists, values(\"false\"))");

    // Check if the resource file exists
    let resource_path = Path::new("static/resource.txt");

    if resource_path.exists() {
        // Set flag indicating the resource file exists
        println!("cargo:rustc-cfg=resource_exists=\"true\"");
        println!("cargo:warning=Resource file found at static/resource.txt");
    } else {
        // Set flag indicating the resource file does not exist
        println!("cargo:rustc-cfg=resource_exists=\"false\"");
        println!("cargo:warning=Resource file not found at static/resource.txt");
    }

    // Always set a flag to indicate build.rs has run
    println!("cargo:rustc-cfg=build_script_run");

    // Ensure the build script reruns if the resource file is added or removed
    println!("cargo:rerun-if-changed=static/resource.txt");
}
