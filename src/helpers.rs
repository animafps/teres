use crate::rendering::CommandWithArgs;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};

use std::io;
use std::io::prelude::*;

pub fn clean(video: PathBuf, script_path: PathBuf) {
    if script_path
        .parent()
        .unwrap()
        .read_dir()
        .unwrap()
        .map(|res| res)
        .collect::<Vec<_>>()
        .len()
        <= 1
    {
        std::fs::remove_dir_all(script_path.parent().unwrap()).unwrap();
    } else {
        std::fs::remove_file(script_path).unwrap();
    }
    std::fs::remove_file(
        video
            .parent()
            .unwrap()
            .join(video.file_name().unwrap().to_str().unwrap().to_owned() + ".ffindex"),
    )
    .unwrap();
}

pub fn exec(ffmpeg_settings: CommandWithArgs) -> ExitStatus {
    let vspipe = Command::new(ffmpeg_settings.vspipe_exe)
        .args(ffmpeg_settings.vspipe_args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start vspipe process");

    let vspipe_out = vspipe.stdout.expect("Failed to open vspipe stdout");

    let ffmpeg = Command::new(ffmpeg_settings.ffmpeg_exe)
        .args(ffmpeg_settings.ffmpeg_args)
        .stdin(Stdio::from(vspipe_out))
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to start ffmpeg process");

    ffmpeg.wait_with_output().unwrap().status
}

pub fn exit(status_code: i32) {
    println!("");
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press enter to close...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let mut stdin = io::stdin(); // We get `Stdin` here.
    stdin.read(&mut [0]).unwrap(); // read_line returns the number of bytes read, so we can ignore it.

    std::process::exit(status_code);
}
