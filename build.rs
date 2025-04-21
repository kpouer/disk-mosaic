#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("media/icon.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {
    // Do nothing on non-Windows platforms
}
