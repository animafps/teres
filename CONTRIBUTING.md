## Install dev environment or manual install

### Requirements
- [Python](https://www.python.org/downloads)
- [FFmpeg](https://ffmpeg.org/download.html)
- [VapourSynth x64](https://www.vapoursynth.com)

### VapourSynth plugins
- [FFMS2](https://github.com/FFMS/ffms2)
- [HAvsFunc](https://github.com/HomeOfVapourSynthEvolution/havsfunc)
- [SVPflow 4.3.0.168](http://www.svp-team.com/files/gpl/svpflow-4.3.0.168.zip)
- [vs-frameblender](https://github.com/f0e/vs-frameblender)
- [weighting.py](https://github.com/f0e/blur/blob/master/plugins/weighting.py)
- [filldrops.py](https://github.com/f0e/blur/blob/master/plugins/filldrops.py)

1. Download [the latest release](https://github.com/animafps/teres/releases/latest) or build the project.
2. Download and run [installer.bat](https://raw.githubusercontent.com/animafps/teres/main/installer.bat) to automatically install all of the requirements.

Or

2. Install Python
3. Install FFmpeg and [add it to PATH](https://www.wikihow.com/Install-FFmpeg-on-Windows)
4. Install the 64-bit version of VapourSynth
5. Install the required VapourSynth plugins using the command "vsrepo.py install ffms2 havsfunc"
6. Install vs-frameblender manually by downloading the x64 .dll from [here](https://github.com/f0e/vs-frameblender/releases/latest) to "VapourSynth/plugins64"
7. Install SVPflow 4.3.0.168 manually by downloading the zip from [here](http://www.svp-team.com/files/gpl/svpflow-4.3.0.168.zip) and moving the files inside "lib-windows/vapoursynth/x64" to "VapourSynth/plugins64"
8. Install [weighting.py](https://raw.githubusercontent.com/f0e/blur/master/plugins/weighting.py) and [filldrops.py](https://github.com/f0e/blur/blob/master/plugins/filldrops.py) to "%appdata%/Roaming/Python/Python39/site-packages"


## Guidelines

- Format with `cargo fmt`
- Follow all guidelines of https://clig.dev
