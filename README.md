# artscii

<!-- prettier-ignore -->
[![crates.io](https://img.shields.io/crates/v/artscii)](https://crates.io/crates/artscii) [![Docs.rs](https://img.shields.io/docsrs/artscii)](https://docs.rs/artscii) [License](https://img.shields.io/crates/l/artscii)

A tool to generate ascii art.

```
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
:::::::::::::::::::::::;:::%;+?%::%+:;::::::::::::::::::::::::
::::::::::::::::::::::;;%%%%%%%%%%%%%%%:;:;:::::::::::::::::::
::::::::::;:::::::;:;%%%%%%%%%%%%%%%%%%%%%::;::::::::::;::::::
::::::;;::%%::::::%%%%%%%%%%%%%%%%%%%%%%%%%%%::::%%%:;%+::::::
::::::?%%:%%%::?%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%:;%%%:%%:::::::
:::::::%%%%%%:::%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%;::%%%%%::::::::
:::::::::*%%::%%%%%%%%%%%%%;@@%%%%%+@%%%%%%%%%%%%:%%::::::::::
:::::::::::?%%%%%%%%%%%%%%S:S@@%%%@:@@%%%%%%%%%%%%::::::::::::
:::::::::::;;%%%%%%%%%%%%%%@@@%%%%@@@%%%%%%%%%%%%%%:::::::::::
::::::::::::%%%:S;SS%%%%%%%%%%%@@@%%%%%%%%SS%:S+?%%:::::::::::
::::::::::::::%%::;;::;:::+SSSSSSSSSS::::;::;?::%:;:::::::::::
::::::::::::::::%::::::::::::::::::::::::::::::%::::::::::::::
:::::::::::::::::::::::::::::::::::::::::::::;%;::::::::::::::
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::

```

## Installation

`artscii` can be used both as a cli tool or as a library in your project.

### Install as CLI tool

#### Pre-built binary

If you want to use it as a CLI tool, head over to the
[releases](https://github.com/theairbend3r/artscii/releases) page.

- Download `artscii-...-.tar.gz` for your preferred platform.
- Extract the file via `tar xzvf artscii-...-.tar.gz`.
- Run `./artscii --path /path/to/img`.

#### Using Cargo

If you have cargo installed on your system.

```bash
cargo install artscii --features cli
```

### Install as library

If you want to use this as a dependency in your project.

```bash
cargo add artscii
```

## Usage

You can convert still images and animations into ASCII art.

Following file formats are currently supported.

| Static | Animation |
| ------ | --------- |
| PNG    | GIF       |
| JPEG   |           |
| JPG    |           |

You can choose between different conversion character sets from default
available `ascii` or `braille` to your custom string.

### As a CLI tool

```bash
artscii --path path/to/file --charset ascii
```

Options

```bash
Usage: artscii [OPTIONS] --path <PATH> --charset <CHARSET>

Options:
  -p, --path <PATH>
  -c, --charset <CHARSET>
  -v, --verbose...         Increase logging verbosity
  -q, --quiet...           Decrease logging verbosity
  -h, --help               Print help
```

### As a library

See `examples/` to get started.

```rust
use std::path::PathBuf;

use artscii::core::{
    canvas::{Canvas, Padding},
    charset::Charset,
    reader::image::ReaderImage,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load image from disk into a Frame
    let path = PathBuf::from("./../test-images/cuddlyferris.png");

    // init canvas, reader, and charset.
    let reader = ReaderImage::new(path);
    let canvas = Canvas::new(210, 53);
    let charset = Charset::new(vec!['⠀', '⠁', '⠃', ':', 'S', '⠏', '#', '⠿', '⡿', '⣿'])?;

    // read and process frame
    let frame = reader.read()?;
    let frame = frame.resize(40, 20)?.to_charset(&charset)?;

    // render frame
    canvas.render(frame, Padding::Center);

    Ok(())
}
```
