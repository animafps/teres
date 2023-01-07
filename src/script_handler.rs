use crate::config::Config;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub fn create(temp_path: PathBuf, video_path: &Path, settings: Config) -> PathBuf {
    let script_filename = temp_path.join(rand::random::<u16>().to_string() + ".vpk");

    let mut script = "from vapoursynth import core\nimport vapoursynth as vs\nimport havsfunc as haf\nimport adjust\nimport weighting\n".to_owned();

    if settings.advanced.encoding.deduplicate {
        script += "import filldrops\n";
    }

    if settings.advanced.interpolation.program == "rife" {
        script += "from vsrife import RIFE\n";
    }

    let extentension = video_path.extension().unwrap().to_str().unwrap();
    if extentension != ".avi" {
        script += format!(
            "video = core.ffms2.Source(source=\"{}\")\n",
            video_path
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
                .replace('\\', "\\\\")
        )
        .as_str();
    } else {
        script += format!(
            "video = core.avisource.AVISource(\"{}\")",
            video_path
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
                .replace('\\', "\\\\")
        )
        .as_str();
        script += "video = core.fmtc.matrix(clip=video, mat=\"709\", col_fam=vs.YUV, bits=16)\n";
        script += "video = core.fmtc.resample(clip=video, css\"420\")\n";
        script += "video = core.fmtc.bitdepth(clip=video, bits=8)\n";
    }

    if settings.timescale.input != 1.0 {
        script += format!(
            "video = core.std.AssumeFPS(video, fpsnum=(video.fps * (1 / {})))",
            settings.timescale.input,
        )
        .as_str();
    }

    if settings.interpolation.enabled {
        if settings.advanced.interpolation.program == "rife" {
            script += "video = core.resize.Bicubic(video, format=vs.RGBS, matrix_in_s=\"709\")\n";
            script += format!("while video.fps < {}:\n", settings.interpolation.fps).as_str();
            script += "    video = RIFE(video)\n";
            script += "video = core.resize.Bicubic(video, format=vs.YUV420P8, matrix_s=\"709\")\n"
        } else if settings.advanced.interpolation.program == "rife-ncnn" {
            script += "video = core.resize.Bicubic(video, format=vs.RGBS, matrix_in_s=\"709\")\n";

            script += format!("while video.fps < {}:\n", settings.interpolation.fps).as_str();
            script += "    video = core.rife.RIFE(video)\n";

            script += "video = core.resize.Bicubic(video, format=vs.YUV420P8, matrix_s=\"709\")\n"
        } else {
            let mut speed = settings.advanced.interpolation.speed;
            if speed.to_lowercase() == "default" {
                speed = "medium".to_string();
            }

            let mut tuning = settings.advanced.interpolation.tuning;
            if tuning.to_lowercase() == "default" {
                tuning = "smooth".to_string();
            }

            let mut algorithm = settings.advanced.interpolation.algorithm;
            if algorithm.to_lowercase() == "default" {
                algorithm = "13".to_string();
            }

            let gpu_bool = if settings.advanced.encoding.gpu {
                "True"
            } else {
                "False"
            };
            script += format!("video = haf.InterFrame(video, GPU={}, NewNum={}, Preset=\"{}\", Tuning=\"{}\", OverrideAlgo={})\n", gpu_bool, settings.interpolation.fps, speed, tuning, algorithm).as_str()
        }
    }

    if settings.timescale.output != 1.0 {
        script += format!(
            "video = core.std.AssumeFPS(video, fpsnum=(video.fps * {}))\n",
            settings.timescale.output,
        )
        .as_str();
    }

    if settings.advanced.encoding.deduplicate {
        script += "video = filldrops.FillDrops(video, thresh=0.001)\n";
    }

    if settings.blending.enabled {
        script += format!(
            "frame_gap = int(video.fps / {})\n",
            settings.blending.output_fps
        )
        .as_str();
        script += format!(
            "blended_frames = int(frame_gap *{})\n",
            settings.blending.amount
        )
        .as_str();

        script += "if blended_frames > 0:\n";
        script += "	if blended_frames % 2 == 0:\n";
        script += "		blended_frames += 1\n";

        let triangle_reverse_bool = if settings.advanced.blend_weighting.triangle_reverse {
            "True"
        } else {
            "False"
        };

        let mut weighting_bound = String::from("[");
        weighting_bound += &settings
            .advanced
            .blend_weighting
            .bound
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        weighting_bound += "]";
        let guassian = format!(
            "weighting.gaussian(blended_frames, {}, {})",
            settings.advanced.blend_weighting.gaussian_std_dev, weighting_bound,
        );
        let gaussian_sym = format!(
            "weighting.gaussianSym(blended_frames, {}, {})",
            settings.advanced.blend_weighting.gaussian_std_dev, weighting_bound
        );
        let pyramid = format!(
            "weighting.pyramid(blended_frames, {})",
            triangle_reverse_bool
        );
        let custom_weight = format!(
            "weighting.divide(blended_frames, {})",
            settings.blending.weighting
        );
        let custom_function = format!(
            "weighting.custom(blended_frames, '{}', {})",
            settings.blending.weighting, weighting_bound
        );
        let weighting_functions = HashMap::from([
            ("equal", "weighting.equal(blended_frames)"),
            ("gaussian", guassian.as_str()),
            ("gaussian_sym", gaussian_sym.as_str()),
            ("pyramid", pyramid.as_str()),
            ("pyramid_sym", "weighting.pyramid(blended_frames)"),
            ("custom_weight", custom_weight.as_str()),
            ("custom_function", custom_function.as_str()),
        ]);

        let mut weighting = settings.blending.weighting;
        if weighting_functions.get(weighting.as_str()).is_none() {
            // check if it's a custom weighting function
            if weighting.starts_with('[') && weighting.ends_with(']') {
                weighting = "custom_weight".to_string();
            } else {
                weighting = "custom_function".to_string();
            }
        }

        script += format!(
            "	weights = {}\n",
            weighting_functions.get(weighting.as_str()).unwrap()
        )
        .as_str();

        script += "	video = core.frameblender.FrameBlend(video, weights, True)\n";

        script += format!(
            "video = haf.ChangeFPS(video, {})\n",
            settings.blending.output_fps
        )
        .as_str();
    }

    if settings.filters.brightness != 1.0
        || settings.filters.contrast != 1.0
        || settings.filters.saturation != 1.0
    {
        script += format!(
            "video = adjust.Tweak(video, bright={}, cont={}, sat={})\n",
            settings.filters.brightness, settings.filters.contrast, settings.filters.saturation
        )
        .as_str();
    }

    script += "video.set_output()\n";
    let file = script_filename.clone();
    std::fs::write(script_filename, script).expect("Could not write script file");
    file
}
