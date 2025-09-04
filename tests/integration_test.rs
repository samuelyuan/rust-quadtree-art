use rust_quadtree_art::quad::{generate_image, subdivide_nodes, Quad, QuadConfig};
use std::fs;

#[test]
fn test_end_to_end_processing() {
    // Create a simple test image
    let mut img = image::RgbaImage::new(64, 64);
    
    // Create a simple pattern: top half red, bottom half blue
    for x in 0..64 {
        for y in 0..32 {
            img.put_pixel(x, y, image::Rgba([255, 0, 0, 255])); // Red
        }
        for y in 32..64 {
            img.put_pixel(x, y, image::Rgba([0, 0, 255, 255])); // Blue
        }
    }
    
    let dynamic_img = image::DynamicImage::ImageRgba8(img);
    let config = QuadConfig {
        max_depth: 3,
        color_threshold: 5.0,
        size_threshold: 8,
        output_file: "test_output.png".to_string(),
    };
    
    // Create initial quad
    let initial_quad = Quad::new(dynamic_img, 0, 0, 64, 64, config.clone());
    
    // Process the quadtree
    let leaves = subdivide_nodes(initial_quad, &config);
    
    // Verify we got some leaves
    assert!(!leaves.is_empty());
    
    // Generate output image
    let result = generate_image(leaves, 64, 64, &config.output_file);
    assert!(result.is_ok());
    
    // Verify output file was created
    assert!(std::path::Path::new(&config.output_file).exists());
    
    // Clean up
    let _ = fs::remove_file(&config.output_file);
}

#[test]
fn test_different_configurations() {
    let mut img = image::RgbaImage::new(32, 32);
    
    // Create a gradient pattern
    for x in 0..32 {
        for y in 0..32 {
            let intensity = ((x + y) * 255 / 64) as u8;
            img.put_pixel(x, y, image::Rgba([intensity, intensity, intensity, 255]));
        }
    }
    
    let dynamic_img = image::DynamicImage::ImageRgba8(img);
    
    // Test with high color threshold (should result in fewer subdivisions)
    let config_high = QuadConfig {
        max_depth: 5,
        color_threshold: 50.0,
        size_threshold: 4,
        output_file: "test_high_threshold.png".to_string(),
    };
    
    let initial_quad_high = Quad::new(dynamic_img.clone(), 0, 0, 32, 32, config_high.clone());
    let leaves_high = subdivide_nodes(initial_quad_high, &config_high);
    
    // Test with low color threshold (should result in more subdivisions)
    let config_low = QuadConfig {
        max_depth: 5,
        color_threshold: 5.0,
        size_threshold: 4,
        output_file: "test_low_threshold.png".to_string(),
    };
    
    let initial_quad_low = Quad::new(dynamic_img, 0, 0, 32, 32, config_low.clone());
    let leaves_low = subdivide_nodes(initial_quad_low, &config_low);
    
    // Low threshold should produce more leaves (more subdivisions)
    assert!(leaves_low.len() > leaves_high.len());
    
    // Clean up
    let _ = fs::remove_file(&config_high.output_file);
    let _ = fs::remove_file(&config_low.output_file);
}
