# blur

blur is a program made for easily and efficiently adding motion blur to videos through frame blending and interpolation.

Based on the work done on <https://github.com/f0e/blur>

## Download/Installation

> Currently blur only supports Windows platforms but support for others is coming

1. Download the setup utility from [the latest release](https://github.com/animafps/blur/releases/latest/)
2. Run setup and follow prompts to install
3. Done! it should be installed 

## Usage

blur can be run from the command line or without

### Non CLI

You can run the program and follow the instructions or use the open with function in explorer

### CLI

e.g.

```bash
blur -n demo.mp4
```

For all cli arguments use

```bash
blur -h
```

### Config

When first run it creates a config file in your home folder. `C:/users/user/blur-config.yml` for example which allows you to change the settings for the interpolation and enconding processes

again same thing applies and that being that not all the properties do anything or sometimes break the program.

## License

This repository is licensed under GPL-3.0-or-later see [LICENSE](LICENSE) for more details
