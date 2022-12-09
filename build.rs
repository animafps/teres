use clap::CommandFactory;

#[path = "src/cli.rs"]
mod cli;

fn man_gen() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR").ok_or_else(|| std::io::ErrorKind::NotFound)?,
    );
    let cmd = cli::Cli::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("teres.1"), buffer)?;

    Ok(())
}

#[cfg(unix)]
fn main() -> std::io::Result<()> {
    man_gen()
}

#[cfg(windows)]
fn main() -> std::io::Result<()> {
    let res = winres::WindowsResource::new();
    res.compile().unwrap();
    man_gen();
    Ok(())
}
