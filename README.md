<h1 align="center">Teres</h1>

<p align="center"> A program made for easily and efficiently adding motion blur or fps to videos through frame blending and interpolation.
    <br> 
</p>

## Features 

- Interpolate video to higher framerate
- Blur frames together for motion blur
- Both CLI and a minimal ui usage
- Supports multiple videos at the same time
- Progress bar
- Global configuration file
- Multithread support

## Installing

> Note: Currently only 64bit Windows is supported but other platforms are in the works

1. Download the setup utility from [the latest release](https://github.com/animafps/teres/releases/latest/)
2. Run setup and follow prompts to install
3. Done! it should be installed 

For manual installation, see [CONTRIBUTING.md](/CONTRIBUTING.md)

## 🎈 Usage

teres can be run from the command line or without

### Non CLI

You can run the program and follow the instructions or use the open with function in explorer

### CLI

e.g.

```bash
teres -n demo.mp4
```

For all cli arguments use

```bash
teres -h
```

### Config

When first run it creates a config file in your home folder. `C:/users/user/teres-config.yml` for example which allows you to change the settings for the interpolation and enconding processes

#### blur
- blur - whether or not the output video file will have motion blur
- blur amount - if blur is enabled, this is the amount of motion blur (0 = no blur, 1 = fully blend every frame together, 1+ = more blur/ghosting)
- blur output fps - if blur is enabled, this is the fps the output video will be
- blur weighting - weighting function to use when blending frames. options are listed below:
  - equal - each frame is blended equally
  - gaussian
  - gaussian_sym
  - pyramid
  - pyramid_sym
  - custom weights - custom frame weights, e.g. [5, 3, 3, 2, 1]. higher numbers indicate frames being more visible when blending, lower numbers mean they are less so.
  - custom function - generate weights based off of custom python code, which is called for each frame 'x', e.g. -x**2+1

#### interpolation
- interpolate - whether or not the input video file will be interpolated to a higher fps
- interpolated fps - if interpolate is enabled, this is the fps that the input file will be interpolated to (before blending)

#### rendering
- quality - [crf](https://trac.ffmpeg.org/wiki/Encode/H.264#crf) of the output video (qp if using GPU rendering)
- preview - opens a render preview window
- detailed filenames - adds blur settings to generated filenames

#### timescale
- input timescale - timescale of the input video file (will be sped up/slowed down accordingly)
- output timescale - timescale of the output video file
- adjust timescaled audio pitch - will pitch shift audio when sped up/slowed down

#### filters
- brightness - brightness of the output video
- saturation - saturation of the output video
- contrast - contrast of the output video

#### advanced rendering
- gpu - enables experimental gpu accelerated rendering (likely slower)
- gpu type (nvidia/amd/intel) - your gpu type
- deduplicate - removes duplicate frames and generates new interpolated frames to take their place
- custom ffmpeg filters - custom ffmpeg filters to be used when rendering (replaces gpu & quality options)

#### advanced blur
- blur weighting gaussian std dev - standard deviation used in the gaussian weighting
- blur weighting triangle reverse - reverses the direction of the triangle weighting
- blur weighting bound - weighting bounds, spreads out weights more

#### advanced interpolation
- interpolation program (svp/rife/rife-ncnn) - program used for interpolation.
  - svp - fastest option, also blurs static parts of video the least
  - rife - considerably slower than SVP but can produce more accurate results, particularly for low framerate input videos. this is the CUDA implementation of RIFE, and is the faster option for NVIDIA gpus.
  - rife-ncnn - Vulkan implementation of rife, works for all devices but is slower.
- interpolation speed - default is 'medium', [explained further here](https://www.spirton.com/uploads/InterFrame/InterFrame2.html)
- interpolation tuning - default is 'smooth', [explained further here](https://www.spirton.com/uploads/InterFrame/InterFrame2.html)
- interpolation algorithm - default is 13, [explained further here](https://www.spirton.com/uploads/InterFrame/InterFrame2.html)

### Recommended settings for gameplay footage:
#### Config options
- blur amount - for maximum blur/smoothness use 1, for medium blur use 0.5, low blur 0.2-0.3. 0.6-0.8 gives nice results for 60fps, 0.3~ is good for 30fps
- blur weighting - just keep it at equal

- interpolated fps - results become worse if this is too high, for 60fps source footage around 300-900 should be good, for 180fps 1200 is good. In general the limit tends to be at around 10x the input video's fps.

- interpolation speed - just keep it at default
- interpolation tuning - for gameplay footage default (smooth) keeps the crosshair intact, but film is also a good option
- interpolation algorithm - just keep it at default

#### Limiting smearing
Using blur on 60fps footage results in clean motion blur, but occasionally leaves some smearing artifacts. To remove these artifacts, higher framerate source footage can be used. Recording with software such as OBS at framerates like 120/180fps will result in a greatly reduced amount of artifacting.

#### Preventing unsmooth output
If your footage contains duplicate frames then occasionally blurred frames will look out of place, making the video seem unsmooth at points. The 'deduplicate' option will automatically fill in duplicated frames with interpolated frames to prevent this from happening.

## ✨ Contributors

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://animafps.xyz"><img src="https://avatars.githubusercontent.com/u/18208134?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Anima</b></sub></a><br /><a href="https://github.com/animafps/teres/commits?author=animafps" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/apps/renovate"><img src="https://avatars.githubusercontent.com/in/2740?v=4?s=100" width="100px;" alt=""/><br /><sub><b>renovate[bot]</b></sub></a><br /><a href="#maintenance-renovate[bot]" title="Maintenance">🚧</a></td>
    <td align="center"><a href="https://github.com/apps/allcontributors"><img src="https://avatars.githubusercontent.com/in/23186?v=4?s=100" width="100px;" alt=""/><br /><sub><b>allcontributors[bot]</b></sub></a><br /><a href="https://github.com/animafps/teres/commits?author=allcontributors[bot]" title="Documentation">📖</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!

## 🎉 Acknowledgements
- [foe's blur](https://github.com/f0e/blur) - Was the basis for the entire code base

## License

This repository is licensed under GPL-3.0-or-later see [LICENSE](LICENSE) for more details
