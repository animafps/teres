use crate::rendering::{Render, Rendering};
use crate::Cli;
use std::process::Command;
use std::vec;
use text_io::read;

pub fn run(cli_args: Cli) {
    let using_ui = !cli_args.noui;

    if using_ui {
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
        let word: String = read!();
        println!("{}", word);
    }

    println!("{}", used_installer());
    if !used_installer() {
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

    let mut rendering = Rendering {
        queue: vec![],
        renders_queued: false,
    };

    if using_ui {
    } else {
        if cli_args.input.is_none() {
            println!("No input file specified");
            std::process::exit(1);
        }
        let input_file = cli_args.input.unwrap();
        if !std::path::Path::new(&input_file).exists() {
            println!("Video {} does not exist", input_file);
            std::process::exit(1);
        }

        let render = Render::new(
            std::fs::canonicalize(input_file).unwrap(),
            cli_args.output,
            using_ui,
        );

        rendering.queue_render(render);
    }

    rendering.render_videos();
}

pub fn create_temp_path(video_path: std::path::PathBuf) -> std::path::PathBuf {
    let temp_path = video_path.join(".blur_temp");
    println!("Creating temp folder: {}", temp_path.display());

    if !temp_path.exists() {
        std::fs::create_dir_all(&temp_path).unwrap();
    }

    temp_path
}

pub fn used_installer() -> bool {
    let path = std::env::current_exe().unwrap();
    let parent_path = path.parent().unwrap();
    println!("{}", parent_path.display());
    parent_path.join("lib/vapoursynth/VSPipe.exe").exists()
        && parent_path.join("lib/ffmpeg/ffmpeg.exe").exists()
}
