use crate::helpers;
use crate::rendering;
use crate::Cli;
use dirs::home_dir;
use rfd::FileDialog;
use std::process::Command;
use std::vec;

pub fn run(cli_args: Cli) -> Option<()> {
    let using_ui = !cli_args.noui;

    let art = vec![
        "    _/        _/                   ",
        "   _/_/_/    _/  _/    _/  _/  _/_/",
        "  _/    _/  _/  _/    _/  _/_/     ",
        " _/    _/  _/  _/    _/  _/        ",
        "_/_/_/    _/    _/_/_/  _/         ",
    ];
    println!();
    for (_, line) in art.iter().enumerate() {
        println!("{}", line);
    }
    println!();

    if cli_args.input.is_none() && !using_ui {
        println!("No video(s) inputted");
        helpers::exit(1);
    }

    if !used_installer().unwrap() {
        let ffmepg = Command::new("ffmpeg").arg("-v").output();

        let python = Command::new("python3").arg("-v").output();

        let vspipe = Command::new("vspipe").arg("-v").output();

        if ffmepg.is_err() {
            println!("FFmpeg is not installed");
        }
        if python.is_err() {
            println!("Python is not installed");
        }
        if vspipe.is_err() {
            println!("VapourSynth is not installed");
        }
    }

    let mut rendering = rendering::Rendering {
        queue: vec![],
        renders_queued: false,
    };

    let files;

    if using_ui {
        println!("Select input video(s)");
        let diag_files = FileDialog::new()
            .add_filter("Video", &["mp4", "mov", "mkv", "avi"])
            .set_directory(home_dir()?.to_str()?)
            .pick_files();
        if diag_files.is_none() {
            println!("No input video(s) selected");
            helpers::exit(1);
        }
        files = diag_files?;
    } else {
        let input = cli_args.input?;
        files = input
            .split(',')
            .map(|file| std::path::Path::new(file).to_path_buf())
            .collect();
    }

    let output = cli_args.output;
    for video in files {
        if !video.exists() {
            println!("Video {} does not exist", video.display());
            helpers::exit(1);
        }
        let render = rendering::Render::new(video, output.clone());
        rendering.queue_render(render?)
    }

    rendering.render_videos();
    Some(())
}

pub fn create_temp_path(
    video_path: std::path::PathBuf,
) -> Result<std::path::PathBuf, std::io::Error> {
    let temp_path = video_path.join(".blur_temp");

    if !temp_path.exists() {
        std::fs::create_dir_all(&temp_path)?;
    }

    Ok(temp_path)
}

pub fn used_installer() -> Result<bool, std::io::Error> {
    let path = std::env::current_exe()?;
    let parent_path = path.parent().unwrap();
    Ok((parent_path.join("lib/vapoursynth/VSPipe.exe").exists()
        && parent_path.join("lib/ffmpeg/ffmpeg.exe").exists())
        || (parent_path.join("lib/vapoursynth/vspipe").exists()
            && parent_path.join("lib/ffmpeg/ffmpeg").exists()))
}
