#[cfg(windows)]
fn main() -> std::io::Result<()> {
    let res = winres::WindowsResource::new();
    res.compile().unwrap();
    Ok(())
}
#[cfg(unix)]
fn main() -> std::io::Result<()> {
    Ok(())
}
