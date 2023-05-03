use std::env;

#[cfg(not(windows))]
fn main() {
    profile_env();
}

#[cfg(windows)]
fn main() {
    profile_env();

    //
    // https://learn.microsoft.com/en-us/cpp/build/reference/export-exports-a-function
    //
    println!("cargo:rustc-cdylib-link-arg=/EXPORT:DetourFinishHelperProcess,@1,NONAME");
}

/// Pass `PROFILE` env to compiler
/// so the correct injection binary
/// build is used
fn profile_env() {
    let profile = env::var("PROFILE").unwrap();
    println!("cargo:rustc-env=PROFILE={profile}");
}
