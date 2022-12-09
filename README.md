<h1 align="center">Teres</h1>

<p align="center"> A program made for easily and efficiently adding motion blur or fps to videos through frame blending and interpolation.
    <br> 
</p>

**NOTE:** This program is unstable and is sub v1.0.0 so has lots of bugs and api/config changes will occur with its development

[![Packaging status](https://repology.org/badge/vertical-allrepos/teres.svg)](https://repology.org/project/teres/versions)

## Features

- Interpolate video to higher framerate
- Blur frames together for motion blur
- Both CLI and a minimal GUI usage
- Supports multiple videos simultatiously
- Progress bar
- Global configuration file
- Multithread and GPU support

## Installing

Use the packages in the repos above

For manual installation, see [CONTRIBUTING.md](/CONTRIBUTING.md)

## 🎈 Usage

Teres can be run from the command line or seperately

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

For configuration options see [the docs](https://animafps.github.io/teres/configuration)

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
- [Smoothie](https://github.com/couleur-tweak-tips/Smoothie) - Couleur and the ctt team helped create more inovation and colaboration with similar goals

## License

This repository is licensed under GPL-3.0-or-later see [LICENSE](LICENSE) for more details
