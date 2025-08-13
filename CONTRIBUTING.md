# Contributing to Teres

We welcome contributions to Teres! This guide will help you get started.

## Development Setup

### Requirements
- [Python](https://www.python.org/downloads)
- [FFmpeg](https://ffmpeg.org/download.html)
- [VapourSynth x64](https://www.vapoursynth.com)

### VapourSynth plugins
- [FFMS2](https://github.com/FFMS/ffms2)
- [HAvsFunc](https://github.com/HomeOfVapourSynthEvolution/havsfunc)
- [SVPflow v4.2.0.142](https://github.com/bjaan/smoothvideo/blob/main/SVPflow_LastGoodVersions.7z)
- [vs-frameblender](https://github.com/f0e/vs-frameblender)
- [weighting.py](https://github.com/f0e/blur/blob/master/plugins/weighting.py)
- [filldrops.py](https://github.com/f0e/blur/blob/master/plugins/filldrops.py)

1. Download [the latest release](https://github.com/animafps/teres/releases/latest) or build the project.
2. Install Python
3. Install FFmpeg and [add it to PATH](https://www.wikihow.com/Install-FFmpeg-on-Windows)
4. Install the 64-bit version of VapourSynth
5. Install the required VapourSynth plugins using the command "vsrepo.py install ffms2 havsfunc"
6. Install vs-frameblender manually by downloading the x64 .dll from [here](https://github.com/f0e/vs-frameblender/releases/latest) to "VapourSynth/plugins64"
7. Install SVPflow v4.2.0.142 manually by downloading the zip from [here](https://github.com/bjaan/smoothvideo/blob/main/SVPflow_LastGoodVersions.7z) and moving the files inside "lib-windows/vapoursynth/x64" to "VapourSynth/plugins64"
8. Install [weighting.py](https://raw.githubusercontent.com/f0e/blur/master/plugins/weighting.py) and [filldrops.py](https://github.com/f0e/blur/blob/master/plugins/filldrops.py) to "%appdata%/Roaming/Python/Python{your python version}/site-packages"


### Building from Source

After installing the requirements above:

```bash
# Clone the repository
git clone https://github.com/animafps/teres.git
cd teres

# Build the project
cargo build --release

# Run tests
cargo test
```

### Documentation Environment

For working on documentation:

```bash
# Install Jekyll dependencies
bundle install

# Serve documentation locally
bundle exec jekyll serve
```

See [jekyll docs](https://jekyllrb.com/docs/) for more information.

## Development Guidelines

### Code Style

- Format code with `cargo fmt` before submitting
- Run `cargo clippy` to check for common mistakes
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Follow CLI guidelines from https://clig.dev

### Commit Messages

- Use conventional commit format: `type(scope): description`
- Examples: `feat: add new blur weighting function`, `fix: resolve memory leak in interpolation`

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-new-feature`
3. Make your changes and test thoroughly
4. Run `cargo fmt` and `cargo clippy`
5. Commit your changes with clear messages
6. Push to your fork and submit a pull request

### Testing

- Add tests for new functionality
- Ensure all tests pass: `cargo test`
- Test on multiple platforms if possible

## Reporting Issues

When reporting bugs, please include:

- Teres version (`teres --version`)
- Operating system and version
- Steps to reproduce the issue
- Expected vs actual behavior
- Log output (use `-v` flag for verbose logs)

## Getting Help

- Join our [Discord server](https://discord.gg/5z3YhWstQr)
- Check existing [GitHub issues](https://github.com/animafps/teres/issues)
- Read the [documentation](https://animafps.github.io/teres/)
