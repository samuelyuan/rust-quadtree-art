use clap::Parser;
use image::GenericImageView;
use std::process;

use quad::{generate_image, subdivide_nodes, Quad, QuadConfig};

mod quad;

#[derive(Parser)]
#[command(name = "rust-quadtree-art")]
#[command(about = "Generate quadtree art from images")]
struct Args {
    /// Input image file
    input: String,
    
    /// Maximum subdivision depth
    #[arg(long, default_value = "7")]
    max_depth: u32,
    
    /// Color distance threshold
    #[arg(long, default_value = "10.0")]
    color_threshold: f64,
    
    /// Minimum quadrant size
    #[arg(long, default_value = "5")]
    size_threshold: u32,
    
    /// Output filename
    #[arg(long, default_value = "output.png")]
    output: String,
}

fn main() {
    let args = Args::parse();
    
    println!("Processing image: {}", args.input);
    
    let img = match image::open(&args.input) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error opening image '{}': {}", args.input, e);
            process::exit(1);
        }
    };

    let (w, h) = img.dimensions();
    println!("Image dimensions: {}x{}", w, h);
    
    let config = QuadConfig {
        max_depth: args.max_depth,
        color_threshold: args.color_threshold,
        size_threshold: args.size_threshold,
        output_file: args.output.clone(),
    };

    let q = Quad::new(img, 0, 0, w, h, config.clone());
    let quadtree_leaves = subdivide_nodes(q, &config);
    
    match generate_image(quadtree_leaves, w, h, &config.output_file) {
        Ok(_) => println!("Successfully generated: {}", config.output_file),
        Err(e) => {
            eprintln!("Error generating output image: {}", e);
            process::exit(1);
        }
    }
}