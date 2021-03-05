#[cfg(target_os = "windows")]
fn main() {
    if cfg!(windows) {
        let mut res = winres::WindowsResource::new();
        res.set_icon("favicon.ico");
        res.compile().expect("Can't compile windows resources");
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
}
