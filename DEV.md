# Development

The smallest unit that can be converted to ASCII is a frame (a matrix of
pixels). A frame can come from any media file such as an image, video, GIF, etc.

## Usage

```bash
artscii --path ./path/to/media.png --width 32 --height 16 --chars unicode --color --invert
```

## IO

### Input

`video/gif` => `frame_{0}, frame_{1}, frame_{2}, ..., frame_{n}`

`image` => `frame_{0}`

### Processing

The program converts the input to ASCII art by replacing the pixel values with
ASCII characters. Some features include:

- Using different image characteristics for converting pixels to ASCII
  characteristics
  - Brightness
  - Sum/average of RGB values
  - ...
- Different ASCII character collections
  - Only letters
  - ASCII
  - Unicode
  - Coloured vs black/white
  - Emojis

### Output

After the conversion happens, the ASCII art is printed on a Canvas i.e. the
terminal.

If there is more than 1 frame, i.e. a video, then the frames are printed in a
loop.

## Objects

### Input

```rust

enum MediaType {
    Image,
    Video
}

Frames {
    path: PathBuf,
    width: u32,
    height: u32,
    aspect_ratio: f32,
    mediaType: MediaType,
    frames: Vec<DynamicImage>, // vector will have only 1 item if it's an image
}
```

Input media is loaded into the structs above.

### Processing

```rust
enum AsciiColourType {
    Gray,
    Rgb
}

enum AsciiCharType {
    Ascii,
    Unicode,
    Alphabet
}


struct Ascii {
    width: u32,
    height: u32,
    color: AsciiColourType,
    char: AsciiCharType,
    art: Vec<Vec<String>>,
}
```

The input media is then processed and stored in the `Ascii` struct.

### Output

```rust
struct Canvas {
    width: u32,
    height: u32,
    aspect_ratio: f32,
}
```

The Canvas struct is used to store the terminal size --- helpful for setting a
maximum size on the ASCII image.
