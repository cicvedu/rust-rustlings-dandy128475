fn main() {
    // Get the current timestamp
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

    // Set the TEST_FOO environment variable
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

