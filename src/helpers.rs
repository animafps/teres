use crate::rendering::{CommandWithArgs, Render};
use indicatif::ProgressBar;
use is_terminal::IsTerminal;
use log::debug;
use regex::Regex;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use std::process::{ChildStderr, Command, ExitStatus, Stdio};

pub fn change_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    result.set_file_name(name);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}

pub fn clean(video: PathBuf, script_path: PathBuf) {
    debug!("Cleaning temp files at: {}", script_path.display());
    if script_path.file_name().unwrap().to_str().unwrap() == ".teres_temp" {
        std::fs::remove_dir_all(script_path).unwrap();
    } else if script_path.parent().unwrap().read_dir().unwrap().count() <= 1 {
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

pub fn clean_temp(videos: Vec<Render>) {
    for video in videos {
        clean(video.video_path, video.video_folder.join(".teres_temp"));
    }
}

pub fn exec(ffmpeg_settings: CommandWithArgs, pb: ProgressBar) -> ExitStatus {
    let vspipe = Command::new(ffmpeg_settings.vspipe_exe)
        .args(ffmpeg_settings.vspipe_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start vspipe process");

    let ffmpeg = Command::new(ffmpeg_settings.ffmpeg_exe)
        .args(ffmpeg_settings.ffmpeg_args)
        .stdin(Stdio::from(
            vspipe.stdout.expect("Failed to open vspipe stdout"),
        ))
        .spawn()
        .expect("Failed to start ffmpeg process");

    debug!("Spawned subprocesses");

    progress(vspipe.stderr.unwrap(), pb);

    ffmpeg.wait_with_output().unwrap().status
}

pub fn exit(status_code: i32) {
    if std::io::stdin().is_terminal() {
        eprintln!();
        let mut stdout = io::stderr();

        // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
        write!(stdout, "Press enter to close...").unwrap();
        stdout.flush().unwrap();

        // Read a single byte and discard
        let mut stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_exact(&mut [0]).unwrap(); // read_line returns the number of bytes read, so we can ignore it.
    }
    std::process::exit(status_code);
}

fn progress(stderr: ChildStderr, progress: ProgressBar) {
    let mut read_frames = false;
    let frame_regex = Regex::new(r"Frame: (?P<current>\d+)/(?P<total>\d+)").unwrap();
    let output_regex = Regex::new(r"Output").unwrap();
    let mut buf = BufReader::new(stderr);

    loop {
        let mut byte_vec = vec![];
        buf.read_until(b'\r', &mut byte_vec).expect("stderr Error");
        let string = String::from_utf8_lossy(&byte_vec);
        if output_regex.is_match(&string) {
            break;
        }
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
