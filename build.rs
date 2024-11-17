use std::process::Command;

fn main() {
    // Record git hash to compile-time environment variable
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    // Record git tag
    let output = Command::new("git")
        .args(["describe", "--tags"])
        .output()
        .unwrap();
    println!(
        "cargo:rustc-env=GIT_TAG={}",
        String::from_utf8(output.stdout).unwrap()
    );
}
