# artscii

<!-- prettier-ignore -->
![crates.io](https://img.shields.io/crates/v/artscii) ![Docs.rs](https://img.shields.io/docsrs/artscii) ![Crates.io License](https://img.shields.io/crates/l/artscii)

A cli tool to generate ascii art.

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

### As a CLI tool

```bash
artscii --path path/to/img
```

Options

```bash
artscii [OPTIONS]
  --path <PATH>  Options:   -p, --path <PATH>
  -v, --verbose...   Increase logging verbosity
  -q, --quiet...     Decrease logging verbosity
  -h, --help         Print help
```

### As a library

```rust
use std::path::PathBuf;

use artscii::core::{
    canvas::{Canvas, Padding},
    decoder::{decoder::Decoder, image::ImageDecoder},
};

fn main() {
    // load image from disk into a Frame
    let path = PathBuf::from("./../test-images/cuddlyferris.png");
    let img_decoder = ImageDecoder::new(path);
    let frame = img_decoder
        .decode()
        .unwrap()
        .resize(40, 20)
        .to_ascii()
        .unwrap();

    // optionally load canvas to print
    let canvas = Canvas::new(210, 53);
    canvas.render(frame, Padding::Center);
}
```
