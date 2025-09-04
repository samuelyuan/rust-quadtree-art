# rust-quadtree-art

## Getting Started

1. Clone the project
2. Run the program
```bash
cargo run -- [input image filename] [options]
```

## Options

- `--max-depth <n>`: Maximum subdivision depth (default: 7)
- `--color-threshold <f>`: Color distance threshold (default: 10.0)
- `--size-threshold <n>`: Minimum quadrant size (default: 5)
- `--output <file>`: Output filename (default: output.png)
- `--log-level <level>`: Log level: error, warn, info, debug, trace (default: info)

## Examples

```bash
# Basic usage
cargo run -- input.jpg

# High detail output
cargo run -- input.jpg --max-depth 10 --color-threshold 5.0

# Custom output
cargo run -- input.jpg --output my_art.png
```

## About

Quadtree art takes an image and recursively divides the image into 4 quadrants until the difference between the average color and each pixel is less than the color distance threshold or the size of each quadrant is smaller than the image dimension threshold.

The result will be a target image that approximates the source image.

Input:
<div style="display:inline-block;">
<img src="https://raw.githubusercontent.com/samuelyuan/rust-quadtree-art/master/screenshots/city_input.jpg" />
</div>

Output:

<div style="display:inline-block;">
<img src="https://raw.githubusercontent.com/samuelyuan/rust-quadtree-art/master/screenshots/city_output.png" />
</div>