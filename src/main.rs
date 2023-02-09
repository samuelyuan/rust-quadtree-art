use image::GenericImageView;
use std::env;

use quad::{generate_image, subdivide_nodes, Quad};

mod quad;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("File path: {}", file_path);
    let img = image::open(file_path).expect("File not found!");

    let (w, h) = img.dimensions();
    let q = Quad::new(img.clone(), 0, 0, w, h, 7, 0);

    let quadtree_leaves = subdivide_nodes(q);
    generate_image(quadtree_leaves, w, h);
}
