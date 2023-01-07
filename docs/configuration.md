---
title: Configuration
layout: default
permalink: /docs/configuration
---

# Configuration of Teres

When first run it creates a config file in the `.config/teres/` folder. `C:/users/user/.config/teres/teres.toml` for example for windows which allows you to change the settings for the interpolation and enconding processes

## Options

### blur

- **blur** - whether or not the output video file will have motion blur
- **blur_amount** - if blur is enabled, this is the amount of motion blur (0 = no blur, 1 = fully blend every frame together, 1+ = more blur/ghosting)
- **blur_output_fps** - if blur is enabled, this is the fps the output video will be
- **blur_weighting** - weighting function to use when blending frames. options are listed below:
  - `equal` - each frame is blended equally
  - `gaussian`
  - `gaussian_sym`
  - `pyramid`
  - `pyramid_sym`
  - custom weights - custom frame weights, e.g. `[5, 3, 3, 2, 1]`. higher numbers indicate frames being more visible when blending, lower numbers mean they are less so.
  - custom function - generate weights based off of custom python code, which is called for each frame 'x', e.g. `-x\*\*2+1`

### interpolation

- **interpolate** - whether or not the input video file will be interpolated to a higher fps
- **interpolated_fps** - if interpolate is enabled, this is the fps that the input file will be interpolated to (before blending)

### rendering

- **quality** - [crf](https://trac.ffmpeg.org/wiki/Encode/H.264#crf) of the output video (qp if using GPU rendering)
- **preview** - opens a render preview window
- **detailed_filenames** - adds blur settings to generated filenames

### timescale

- **input_timescale** - timescale of the input video file (will be sped up/slowed down accordingly)
- **output_timescale** - timescale of the output video file
- **adjust_timescaled_audio_pitch** - will pitch shift audio when sped up/slowed down

### filters

- **brightness** - brightness of the output video
- **saturation** - saturation of the output video
- **contras**t - contrast of the output video

### advanced rendering

- **gpu** - enables experimental gpu accelerated rendering (likely slower)
- **gpu_type** (nvidia/amd/intel) - your gpu type
- **deduplicate** - removes duplicate frames and generates new interpolated frames to take their place
- **custom_ffmpeg_filters** - custom ffmpeg filters to be used when rendering (replaces gpu & quality options)

### advanced blur

- **blur_weighting_gaussian_std_dev** - standard deviation used in the gaussian weighting
- **blur_weighting_triangle_reverse** - reverses the direction of the triangle weighting
- **blur_weighting_bound** - weighting bounds, spreads out weights more

### advanced interpolation

- **interpolation_program** (svp/rife/rife-ncnn) - program used for interpolation.
  - `svp` - fastest option, also blurs static parts of video the least
  - `rife` - considerably slower than SVP but can produce more accurate results, particularly for low framerate input videos. this is the CUDA implementation of RIFE, and is the faster option for NVIDIA gpus.
  - `rife-ncnn` - Vulkan implementation of rife, works for all devices but is slower.
- **interpolation_speed** - default is `medium`, [explained further here](https://www.spirton.com/uploads/InterFrame/InterFrame2.html) (used in svp)
- **interpolation_tuning** - default is `smooth`, [explained further here](https://www.spirton.com/uploads/InterFrame/InterFrame2.html) (used in svp)
- **interpolation_algorithm** - default is `13`, [explained further here](https://www.spirton.com/uploads/InterFrame/InterFrame2.html) (used in svp)

## Recommended settings for gameplay footage

### Config options

- blur amount - for maximum blur/smoothness use 1, for medium blur use 0.5, low blur 0.2-0.3. 0.6-0.8 gives nice results for 60fps, 0.3~ is good for 30fps
- blur weighting - just keep it at equal

- interpolated fps - results become worse if this is too high, for 60fps source footage around 300-900 should be good, for 180fps 1200 is good. In general the limit tends to be at around 10x the input video's fps.

- interpolation speed - just keep it at default
- interpolation tuning - for gameplay footage default (smooth) keeps the crosshair intact, but film is also a good option
- interpolation algorithm - just keep it at default

### Limiting smearing

Using blur on 60fps footage results in clean motion blur, but occasionally leaves some smearing artifacts. To remove these artifacts, higher framerate source footage can be used. Recording with software such as OBS at framerates like 120/180fps will result in a greatly reduced amount of artifacting.

### Preventing unsmooth output

If your footage contains duplicate frames then occasionally blurred frames will look out of place, making the video seem unsmooth at points. The 'deduplicate' option will automatically fill in duplicated frames with interpolated frames to prevent this from happening.