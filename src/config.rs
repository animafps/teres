use dirs::home_dir;
use std::{fs, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub blending: Blending,
    pub interpolation: Interpolation,
    pub encoding: Encoding,
    pub timescale: Timescale,

    pub filters: Filters,

    pub advanced: Advanced,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blending {
    pub enabled: bool,
    pub amount: f32,
    pub weighting: String,
    pub output_fps: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interpolation {
    pub enabled: bool,
    pub fps: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Encoding {
    pub quality: i32,
    pub detailed_filename: bool,
    pub container: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Timescale {
    pub input: f32,
    pub output: f32,
    pub adjust_audio_pitch: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Filters {
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Advanced {
    pub encoding: AdvancedEncoding,
    pub blend_weighting: AdvancedBlending,
    pub interpolation: AdvancedInterpolation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdvancedEncoding {
    pub gpu: bool,
    pub gpu_type: String,
    pub deduplicate: bool,
    pub custom_ffmpeg_filters: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdvancedBlending {
    pub gaussian_std_dev: i32,
    pub triangle_reverse: bool,
    pub bound: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdvancedInterpolation {
    pub program: String,
    pub speed: String,
    pub tuning: String,
    pub algorithm: String,
}

impl Config {
    pub fn parse() -> Config {
        let filepath = home_dir().unwrap();
        let config_file = filepath.join(".config/teres/teres.toml");
        if !config_file.exists() {
            Config::create(&config_file);
        }

        let mut f = std::fs::File::open(config_file).expect("Could not open file.");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Could not parse config file to string");
        let scrape_config: Config = toml::from_str(&contents).expect("Could not read values.");
        scrape_config
    }

    pub fn create(filepath: &std::path::Path) {
        let prefix = filepath.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        fs::write(filepath, String::from("# Teres Configuration
# For documentation for what each value means and the accecpted values see
# https://animafps.github.io/teres/docs/configuration

[blending]
enabled = true
amount = 1.0
weighting = \"equal\" # equal/gaussian/gaussian_sym/pyramid/pyramid_sym
output_fps = 60

[interpolation]
enabled = true
fps = 480.0

[encoding]
quality = 18
detailed_filename = false
container = \"mp4\"

[timescale]
input = 1.0
output = 1.0
adjust_audio_pitch = false

[filters]
brightness = 1.0
contrast = 1.0
saturation = 1.0

[advanced.encoding]
gpu = false
gpu_type = \"nvidia\" # nvidia/intel/amd
deduplicate = false

[advanced.blend_weighting]
gaussian_std_dev = 2
triangle_reverse = false
bound = [0, 2]

[advanced.interpolation]
program = \"svp\" # svp/rife/rife-ncnn
speed = \"default\" # medium/fast/faster/default (default is medium)
tuning = \"default\" # film/animation/weak/smooth/default (default is smooth)
algorithm = \"default\" # 2/13/23/default (default is 13)"))
            .expect("Failed to create config file")
    }
}
