use crate::cli::Cli;
use crate::helpers;
use crate::helpers::exit;
use crate::rendering;
use dirs::home_dir;
use rfd::FileDialog;
use std::process::{self, Command};
use std::vec;

pub fn run(cli_args: Cli) -> Option<()> {
    let using_ui = !cli_args.noui;

    let art = vec![
        "    ████████╗███████╗██████╗ ███████╗███████╗",
        "    ╚══██╔══╝██╔════╝██╔══██╗██╔════╝██╔════╝",
        "       ██║   █████╗  ██████╔╝█████╗  ███████╗",
        "       ██║   ██╔══╝  ██╔══██╗██╔══╝  ╚════██║",
        "       ██║   ███████╗██║  ██║███████╗███████║",
        "       ╚═╝   ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝",
    ];
    eprintln!();
    for (_, line) in art.iter().enumerate() {
        eprintln!("{}", line);
    }
    eprintln!();

    if cli_args.input.is_empty() && !using_ui {
        error!("No video(s) inputted");
        exit(exitcode::NOINPUT);
    }

    if (!used_installer().unwrap() && cfg!(target_os = "windows")) || cfg!(target_family = "unix") {
        let ffmepg = Command::new("ffmpeg").arg("-v").output();

        let python = Command::new("python3").arg("-v").output();

        let vspipe = Command::new("vspipe").arg("-v").output();

        if ffmepg.is_err() {
            error!("FFmpeg is not installed");
            exit(exitcode::UNAVAILABLE)
        }
        if python.is_err() {
            error!("Python is not installed");
            exit(exitcode::UNAVAILABLE)
        }
        if vspipe.is_err() {
            error!("VapourSynth is not installed");
            exit(exitcode::UNAVAILABLE)
        }
    }

    let mut rendering = rendering::Rendering {
        queue: vec![],
        renders_queued: false,
    };

    let files = if cli_args.input.is_empty() {
        eprintln!("Select input video(s)");
        let diag_files = FileDialog::new()
            .add_filter("Video", &["mp4", "mov", "mkv", "avi"])
            .set_directory(home_dir()?.to_str()?)
            .pick_files();
        if diag_files.is_none() {
            error!("No input video(s) selected");
            exit(exitcode::NOINPUT);
        }
        diag_files?
    } else {
        let input = cli_args.input;
        input
            .iter()
            .map(|file| std::path::Path::new(file).to_path_buf())
            .collect()
    };

    for video in files {
        if !video.exists() {
            error!("Video {} does not exist", video.display());
            exit(exitcode::NOINPUT);
        }
        let render = rendering::Render::new(video);
        rendering.queue_render(render?)
    }

    let clone = rendering.clone().queue;

    ctrlc::set_handler(move || {
        helpers::clean_temp(clone.to_vec());
        process::exit(exitcode::OK)
    })
    .expect("Error setting Ctrl-C handler");

    debug!("Queued renders");
    rendering.render_videos();
    Some(())
}

pub fn create_temp_path(
    video_path: std::path::PathBuf,
) -> Result<std::path::PathBuf, std::io::Error> {
    let temp_path = video_path.join(".teres_temp");

    if !temp_path.exists() {
        std::fs::create_dir_all(&temp_path)?;
    }

    Ok(temp_path)
}

#[cfg(target_os = "windows")]
pub fn used_installer() -> Result<bool, std::io::Error> {
    let path = std::env::current_exe()?;
    let parent_path = path.parent().unwrap();
    Ok((parent_path.join("lib/vapoursynth/VSPipe.exe").exists()
        && parent_path.join("lib/ffmpeg/ffmpeg.exe").exists())
        || (parent_path.join("lib/vapoursynth/vspipe").exists()
            && parent_path.join("lib/ffmpeg/ffmpeg").exists()))
}

#[cfg(target_family = "unix")]
pub fn used_installer() -> Result<bool, std::io::Error> {
    Ok(false)
}
