#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn main() {
    //
    // https://learn.microsoft.com/en-us/cpp/build/reference/export-exports-a-function
    //
    println!("cargo:rustc-cdylib-link-arg=/EXPORT:DetourFinishHelperProcess,@1,NONAME");
}
