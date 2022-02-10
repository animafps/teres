use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use regex::Regex;

use crate::rendering::CommandWithArgs;
use std::path::PathBuf;
use std::process::{ChildStderr, Command, ExitStatus, Stdio};

use std::io::prelude::*;
use std::io::{self, BufReader};
use std::sync::Arc;

pub fn clean(video: PathBuf, script_path: PathBuf) {
    if script_path.parent().unwrap().read_dir().unwrap().count() <= 1 {
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

pub fn exec(
    ffmpeg_settings: CommandWithArgs,
    m: Arc<MultiProgress>,
    video_filename: String,
) -> ExitStatus {
    let vspipe = Command::new(ffmpeg_settings.vspipe_exe)
        .args(ffmpeg_settings.vspipe_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start vspipe process");

    let vspipe_out = vspipe.stdout.expect("Failed to open vspipe stdout");
    let vspipe_stderr = vspipe.stderr.expect("Failed to open vspipe stderr");

    progress(vspipe_stderr, m, video_filename);

    let ffmpeg = Command::new(ffmpeg_settings.ffmpeg_exe)
        .args(ffmpeg_settings.ffmpeg_args)
        .stdin(Stdio::from(vspipe_out))
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start ffmpeg process");

    ffmpeg.wait_with_output().unwrap().status
}

pub fn exit(status_code: i32) {
    println!();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press enter to close...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let mut stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_exact(&mut [0]).unwrap(); // read_line returns the number of bytes read, so we can ignore it.

    std::process::exit(status_code);
}

fn progress(stderr: ChildStderr, progress_bar: Arc<MultiProgress>, video_filename: String) {
    let mut stderr = BufReader::new(stderr);
    let mut read_frames = false;
    let frame_regex = Regex::new(r"Frame: (?P<current>\d+)/(?P<total>\d+)").unwrap();
    let progress = progress_bar.add(ProgressBar::new(100));
    progress.set_style(
        ProgressStyle::default_bar().template("[{msg}] {wide_bar} {percent}% {eta_precise}"),
    );
    progress.set_message(video_filename);

    loop {
        let mut buffer = [0; 32];
        match stderr.read_exact(&mut buffer) {
            Ok(_) => {}
            Err(_) => {
                progress.finish();
                break;
            }
        }
        let string = String::from_utf8_lossy(&buffer);

        let caps;
        if frame_regex.is_match(&string) {
            caps = frame_regex.captures(&string).unwrap();
            if !read_frames {
                progress.set_length(caps["total"].parse::<u64>().unwrap());
                read_frames = true
            }
            progress.set_position(caps["current"].parse::<u64>().unwrap())
        }
    }
}
