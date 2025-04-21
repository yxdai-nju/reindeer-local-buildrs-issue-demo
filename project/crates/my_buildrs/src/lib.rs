pub fn show_status() {
    // Check if build.rs has run
    #[cfg(not(build_script_run))]
    {
        println!("build.rs did not run before compilation");
    }
    
    // Check if build.rs has run and resource exists
    #[cfg(all(build_script_run, resource_exists = "true"))]
    {
        println!("build.rs ran before compilation and successfully detected the static/resource.txt file");
    }
    
    // Check if build.rs has run but resource does not exist
    #[cfg(all(build_script_run, resource_exists = "false"))]
    {
        println!("build.rs ran before compilation but did not detect the static/resource.txt file");
    }
}
