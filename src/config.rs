use dirs::home_dir;
use std::fs;

use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub blur: bool,
    pub blur_amount: f32,
    pub blur_output_fps: f32,
    pub blur_weighting: String,

    pub interpolate: bool,
    pub interpolated_fps: f32,

    pub quality: u32,
    pub detailed_filenames: bool,

    pub input_timescale: f32,
    pub output_timescale: f32,
    pub adjust_timescaled_audio_pitch: bool,

    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,

    pub gpu: bool,
    pub gpu_type: String,
    pub deduplicate: bool,
    pub custom_ffmpeg_filters: String,

    pub blur_weighting_gaussian_std_dev: f32,
    pub blur_weighting_triangle_reverse: bool,
    pub blur_weighting_bound: Vec<f32>,

    pub interpolation_program: String,
    pub interpolation_speed: String,
    pub interpolation_tuning: String,
    pub interpolation_algorithm: String,
}

impl Config {
    pub fn parse() -> Config {
        let filepath = home_dir().unwrap();
        let config_file = filepath.join(".blur-config.yml");
        if !config_file.exists() {
            Config::create(&config_file);
        }

        let f = std::fs::File::open(config_file).expect("Could not open file.");
        let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
        scrape_config
    }

    pub fn create(filepath: &std::path::Path) {
        fs::write(
            filepath,
            "# blur
blur: true
blur_amount: 1
blur_output_fps: 60
blur_weighting: equal

# interpolation
interpolate: true
interpolated_fps: 480

# rendering
quality: 18
detailed_filenames: false

# timescale
input_timescale: 1
output_timescale: 1
adjust_timescaled_audio_pitch: false

# filters
brightness: 1
saturation: 1
contrast: 1

# advanced rendering
gpu: false
gpu_type: nvidia #nvidia/amd/intel
deduplicate: false
custom_ffmpeg_filters: 

# advanced blur
blur_weighting_gaussian_std_dev: 2
blur_weighting_triangle_reverse: false
blur_weighting_bound: [0, 2]

# advanced interpolation
interpolation_program: svp #svp/rife/rife-ncnn
interpolation_speed: default
interpolation_tuning: default
interpolation_algorithm: default",
        )
        .expect("Failed to create config file")
    }
}
