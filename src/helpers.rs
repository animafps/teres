use crate::rendering::CommandWithArgs;
use std::process::{Command, Stdio};

pub fn clean() {}

pub fn exec(ffmpeg_settings: CommandWithArgs) {
    let vspipe = Command::new(ffmpeg_settings.vspipe_exe)
        .args(ffmpeg_settings.vspipe_args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start vspipe process");

    let vspipe_out = vspipe.stdout.expect("Failed to open vspipe stdout");

    let ffmpeg = Command::new(ffmpeg_settings.ffmpeg_exe)
        .args(ffmpeg_settings.ffmpeg_args)
        .stdin(Stdio::from(vspipe_out))
        .spawn()
        .expect("Failed to start ffmpeg process");
}
