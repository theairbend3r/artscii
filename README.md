# artscii

<!-- prettier-ignore -->
![crates.io](https://img.shields.io/crates/v/artscii) ![Docs.rs](https://img.shields.io/docsrs/artscii) ![Crates.io License](https://img.shields.io/crates/l/artscii)

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

### As a CLI tool

```bash
artscii --path path/to/file
```

Options

```bash
Usage: artscii [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>
  -v, --verbose...   Increase logging verbosity
  -q, --quiet...     Decrease logging verbosity
  -h, --help         Print help

```

### As a library

See `examples/` to get started.

```rust
use std::path::PathBuf;

use artscii::core::{
    canvas::{Canvas, Padding},
    decoder::{decoder::Decoder, image::ImageDecoder},
};

fn main() {
    // load image from disk into a Frame
    let path = PathBuf::from("./path/to/image");
    let img_decoder = ImageDecoder::new(path).decode().unwrap();
    let frame = img_decoder.resize(40, 20).to_ascii().unwrap();

    // optionally load canvas to print
    let canvas = Canvas::new(210, 53);
    canvas.render(frame, Padding::Center);
}
```
