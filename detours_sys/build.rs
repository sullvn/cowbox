#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn main() {
    vcpkg::find_package("detours").unwrap();
}
