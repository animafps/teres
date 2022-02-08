use crate::blur::{create_temp_path, used_installer};
use crate::config::Config;
use crate::helpers::{clean, exec};
use crate::script_handler::create;
use std::path::PathBuf;
use std::vec::Vec;

pub struct Render {
    video_name: String,
    video_path: PathBuf,
    video_folder: PathBuf,

    input_filename: String,
    output_filepath: PathBuf,

    settings: Config,
}

impl Render {
    pub fn new(input_path: PathBuf, output_path: Option<String>, using_ui: bool) -> Render {
        let video_folder = input_path.parent().unwrap().to_path_buf();
        let video_path = input_path;

        let video_name = video_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let input_filename = video_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let settings = Config::parse();
        let output_filepath;
        if output_path.is_none() {
            output_filepath = video_folder.join(format!("{}_blur.mp4", video_name));
        } else {
            output_filepath = PathBuf::from(output_path.unwrap());
        }

        Render {
            video_path,
            video_name,
            video_folder,
            input_filename,
            output_filepath,
            settings,
        }
    }
}

impl PartialEq for Render {
    fn eq(&self, other: &Self) -> bool {
        self.video_path == other.video_path
    }
}

pub struct Rendering {
    pub queue: Vec<Render>,
    pub renders_queued: bool,
}

pub struct CommandWithArgs {
    pub ffmpeg_exe: String,
    pub ffmpeg_args: Vec<String>,

    pub vspipe_exe: String,
    pub vspipe_args: Vec<String>,
}

impl Rendering {
    pub fn queue_render(&mut self, render: Render) {
        self.queue.push(render);
        self.renders_queued = true;
    }

    pub fn render_videos(&mut self) {
        if self.renders_queued {
            for render in self.queue.iter() {
                let output_filepath = render.output_filepath.clone();
                let input_filename = render.input_filename.clone();
                let settings = render.settings.clone();
                let video_path = render.video_path.clone();
                let video_folder = render.video_folder.clone();
                let video_name = render.video_name.clone();
                if render != self.queue.last().unwrap() {
                    std::thread::spawn(move || {
                        Rendering::render_video(
                            input_filename,
                            output_filepath,
                            settings,
                            video_path,
                            video_folder,
                            video_name,
                        );
                    });
                } else {
                    Rendering::render_video(
                        input_filename,
                        output_filepath,
                        settings,
                        video_path,
                        video_folder,
                        video_name,
                    );
                }
            }
            clean();
            self.queue.clear();
            self.renders_queued = false;
        }
    }

    pub fn render_video(
        input_filename: String,
        output_filepath: PathBuf,
        settings: Config,
        video_path: PathBuf,
        video_folder: PathBuf,
        video_name: String,
    ) {
        println!("Rendering {}", video_name);

        let temp_path = create_temp_path(video_folder);

        let video_clone = video_path.clone();
        let settings_clone = settings.clone();
        let script_path = create(temp_path, video_path, settings);

        let ffmpeg_settings = Rendering::build_ffmpeg_command(
            script_path,
            video_clone,
            output_filepath,
            settings_clone,
        );
        exec(ffmpeg_settings);
    }

    pub fn build_ffmpeg_command(
        script_path: PathBuf,
        video_path: PathBuf,
        output_path: PathBuf,
        settings: Config,
    ) -> CommandWithArgs {
        let mut vspipe_path = "vspipe";
        let mut ffmpeg_path = "ffmpeg";
        let vspipe_exe;
        let ffmpeg_exe;

        if used_installer() {
            let exepath = std::env::current_exe().unwrap();
            let path = exepath.parent().unwrap();
            println!("{}", path.display());
            vspipe_exe = format!("{}/lib/vapoursynth/VSPipe.exe", path.to_str().unwrap());
            vspipe_path = vspipe_exe.as_str();
            ffmpeg_exe = format!("{}/lib/ffmpeg/ffmpeg.exe", path.to_str().unwrap());
            ffmpeg_path = ffmpeg_exe.as_str();
        }

        let pipe_args = vec![
            "-y".to_string(),
            script_path.to_str().unwrap().to_string(),
            "-".to_string(),
        ];

        let mut ffmpeg_command = String::new();

        ffmpeg_command += "-loglevel error -hide_banner -stats";

        // input
        ffmpeg_command += " -i pipe:"; // piped output from video script
        ffmpeg_command += format!(
            " -i {}",
            video_path.to_str().unwrap().trim_start_matches("\\\\?\\")
        )
        .as_str(); // original video (for audio)
        ffmpeg_command += " -map 0:v -map 1:a?"; // copy video from first input, copy audio from second

        // audio filters
        let mut audio_filters = String::new();
        if settings.input_timescale != 1.0 {
            // asetrate: speed up and change pitch
            audio_filters +=
                format!("asetrate=48000*{}", (1.0 / settings.input_timescale)).as_str();
        }

        if settings.output_timescale != 1.0 {
            if !audio_filters.is_empty() {
                audio_filters += ",";
            }
            if settings.adjust_timescaled_audio_pitch {
                audio_filters += format!("asetrate=48000*{}", settings.output_timescale).as_str();
            } else {
                // atempo: speed up without changing pitch
                audio_filters += format!("atempo={}", settings.output_timescale).as_str();
            }
        }
        if !audio_filters.is_empty() {
            ffmpeg_command += format!(" -af {}", audio_filters).as_str();
        }

        if !settings.custom_ffmpeg_filters.is_empty() {
            ffmpeg_command += format!(" {}", settings.custom_ffmpeg_filters).as_str();
        } else {
            // video format
            if settings.gpu {
                if settings.gpu_type.to_lowercase() == "nvidia" {
                    ffmpeg_command +=
                        format!(" -c:v h264_nvenc -preset p7 -qp {}", settings.quality).as_str();
                } else if settings.gpu_type.to_lowercase() == "amd" {
                    ffmpeg_command += format!(
                        " -c:v h264_amf -qp_i {} -qp_b {} -qp_p {} -quality quality",
                        settings.quality, settings.quality, settings.quality,
                    )
                    .as_str();
                } else if settings.gpu_type.to_lowercase() == "intel" {
                    ffmpeg_command += format!(
                        " -c:v h264_qsv -global_quality {} -preset veryslow",
                        settings.quality,
                    )
                    .as_str();
                }
            } else {
                ffmpeg_command += format!(
                    " -c:v libx264 -pix_fmt yuv420p -preset superfast -crf {}",
                    settings.quality,
                )
                .as_str();
            }

            // audio format
            ffmpeg_command += " -c:a aac -b:a 320k";

            // extra
            ffmpeg_command += " -movflags +faststart";
        }

        // output
        ffmpeg_command += format!(
            " {}",
            output_path.to_str().unwrap().trim_start_matches("\\\\?\\")
        )
        .as_str();
        let ffmpeg_args: Vec<String> = ffmpeg_command
            .split(' ')
            .map(|x| x.to_string())
            .filter(|n| n != "~")
            .collect();
        CommandWithArgs {
            ffmpeg_exe: ffmpeg_path.to_string(),
            ffmpeg_args,

            vspipe_exe: vspipe_path.to_string(),
            vspipe_args: pipe_args,
        }
    }
}
