# rust-quadtree-art

## Getting Started

1. Clone the project
2. Run the program
```
cargo run [input image filename]
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