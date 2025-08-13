<div align="center">

# Teres

[![Discord](https://img.shields.io/discord/1054176051498078218?style=flat-square)](https://discord.gg/5z3YhWstQr) ![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/animafps/teres/ci.yml?branch=main&style=flat-square)
 
*Teres* is a program made to easily and efficiently add motion blur or frames to videos through frame blending and interpolation.

> **NOTE:** This program is unstable and is sub v1.0.0 so has lots of bugs and api/config changes will occur with its development

---

| Before | After Teres |
| --- | --- |
| ![without](./docs/demo.gif) | ![with teres](./docs/demo_blur.gif) |

</div>

*60fps => 960fps(rife) => 60fps (blur amount 1)*

[![Packaging status](https://repology.org/badge/vertical-allrepos/teres.svg)](https://repology.org/project/teres/versions)

## Features

- Interpolate video to higher framerate
- Blur frames together for motion blur
- Both CLI and a minimal GUI usage
- Supports multiple videos simultaneously
- Progress bar
- Global configuration file
- Multiple file queue

## Installing

### Package Managers

Use the packages in the repos above for your distribution.

### Pre-built Binaries

Download the latest release from [GitHub Releases](https://github.com/animafps/teres/releases/latest).

### Manual Installation

For building from source, see [CONTRIBUTING.md](/CONTRIBUTING.md).

## System Requirements

- **Operating System**: Windows 10+, or Linux
- **Dependencies**:
  - [Python 3.7+](https://www.python.org/downloads)
  - [FFmpeg](https://ffmpeg.org/download.html)
  - [VapourSynth x64](https://www.vapoursynth.com)

See [CONTRIBUTING.md](/CONTRIBUTING.md) for detailed dependency installation instructions.

## 🎈 Usage

[**Support and development Discord**](https://discord.gg/5z3YhWstQr)

Teres can be run from the command line or separately

### Non CLI

You can run the program and follow the instructions or use the "open with" function in explorer

### CLI

```
USAGE:
    teres [OPTIONS] [INPUT]...

ARGS:
    [INPUT]...    Input file name(s) (space separated) or glob pattern

OPTIONS:
    -n, --noui       Disable user interface (CLI only)
    -v, --verbose... More output per occurence
    -q, --quiet...   Less output per occurence
    -h, --help       Print help information
    -V, --version    Print version information
```

#### Examples

```bash
# Process a single video file
teres input.mp4

# Process multiple files
teres video1.mp4 video2.mp4 video3.mp4

# Process all MP4 files in current directory
teres *.mp4

# CLI mode with verbose output
teres -n -v input.mp4
```

For configuration options see [the docs](https://animafps.github.io/teres/docs/configuration).

## Troubleshooting

### Getting Help

- Join our [Discord server](https://discord.gg/5z3YhWstQr) for community support
- Check [existing issues](https://github.com/animafps/teres/issues) on GitHub
- For bugs, create a [new issue](https://github.com/animafps/teres/issues/new) with your log output

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

See [CONTRIBUTING.md](./CONTRIBUTING.md), [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) and [active issues](https://github.com/animafps/teres/issues) if you want to contribute

## 🎉 Acknowledgements

- [foe's blur](https://github.com/f0e/blur) - Was the basis for the entire code base
- [Smoothie](https://github.com/couleur-tweak-tips/Smoothie) - Couleur and the ctt team helped create more inovation and colaboration with similar goals
- [vs-frameblender](https://github.com/couleurm/vs-frameblender) - Plugin used for blending the resulting frames of interpolation
- [ffmpeg](https://ffmpeg.org/) - The program used for encoding the interpreted frames
- [vapoursynth](https://www.vapoursynth.com) - Program for manipulating and interpolating videos
- [ffms2](https://github.com/FFMS/ffms2) - Plugin for inputting the video file
- [havsfunc](https://github.com/HomeOfVapourSynthEvolution/havsfunc) - Plugin that provides the svp interpolation function
- [mvsfunc](https://github.com/HomeOfVapourSynthEvolution/mvsfunc) - Plugin that provides colour manipulation
- [vs-rife](https://github.com/HolyWu/vs-rife) - Vapoursynth implementation of the interpolation algorithm RIFE
- [VapourSynth-RIFE-ncnn-Vulkan](https://github.com/HomeOfVapourSynthEvolution/VapourSynth-RIFE-ncnn-Vulkan) - Vulkan implementation of RIFE

## License

This repository is licensed under GPL-3.0-or-later see [COPYING](COPYING) for more details
