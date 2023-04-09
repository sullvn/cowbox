#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn main() {
    vcpkg::find_package("detours").unwrap();

    //
    // https://learn.microsoft.com/en-us/cpp/build/reference/export-exports-a-function
    //
    println!("cargo:rustc-cdylib-link-arg=/EXPORT:DetourFinishHelperProcess,@1,NONAME");
}
