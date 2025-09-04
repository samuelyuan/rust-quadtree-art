use image::{GenericImageView, Pixel, Rgba, RgbaImage};
use imageproc::drawing::draw_line_segment_mut;
use std::collections::VecDeque;
use std::error::Error;

/// Configuration parameters for quadtree generation
#[derive(Clone, Debug)]
pub struct QuadConfig {
    pub max_depth: u32,
    pub color_threshold: f64,
    pub size_threshold: u32,
    pub output_file: String,
}

impl Default for QuadConfig {
    fn default() -> Self {
        Self {
            max_depth: 7,
            color_threshold: 10.0,
            size_threshold: 5,
            output_file: "output.png".to_string(),
        }
    }
}

/// Represents a quadrant in the quadtree structure
#[derive(Clone)]
pub struct Quad {
    image: std::rc::Rc<image::DynamicImage>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: Rgba<u8>,
    config: QuadConfig,
    cur_depth: u32,
}

/// Subdivides the initial quad into a quadtree structure based on color variance
pub fn subdivide_nodes(initial_quad: Quad, config: &QuadConfig) -> Vec<Quad> {
    let mut deque: VecDeque<Quad> = VecDeque::new();
    deque.push_back(initial_quad.clone());
    let mut quadtree_leaves: Vec<Quad> = Vec::new();
    
    // Safety limit to prevent excessive memory usage
    const MAX_QUADS: usize = 100_000;

    while let Some(mut next_quad) = deque.pop_front() {
        // Safety check
        if quadtree_leaves.len() > MAX_QUADS {
            eprintln!("Reached maximum quad limit, stopping subdivision");
            break;
        }
        
        // Skip invalid quads
        if next_quad.x >= initial_quad.width || next_quad.y >= initial_quad.height {
            continue;
        }
        
        // Check if we should subdivide this quad
        if should_subdivide(&next_quad, config) {
            let children = next_quad.subdivide();
            for child_node in children {
                deque.push_back(child_node);
            }
        } else {
            // Calculate average color for this leaf
            next_quad.color = next_quad.calc_avg_color();
            quadtree_leaves.push(next_quad);
        }
    }
    
    quadtree_leaves
}

/// Determines if a quad should be subdivided based on depth, color variance, and size
fn should_subdivide(quad: &Quad, config: &QuadConfig) -> bool {
    quad.cur_depth < config.max_depth
        && quad.calc_color_distance() > config.color_threshold
        && quad.width > config.size_threshold
        && quad.height > config.size_threshold
}

/// Generates the output image from quadtree leaf nodes
pub fn generate_image(quadtree_leaves: Vec<Quad>, image_width: u32, image_height: u32, output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut output_image = RgbaImage::new(image_width, image_height);
    let black = Rgba([0, 0, 0, 255]);

    for leaf in quadtree_leaves {
        // Fill the quad with its average color
        fill_quad_with_color(&mut output_image, &leaf);
        
        // Draw outline around the quad
        draw_quad_outline(&mut output_image, &leaf, black);
    }
    
    output_image.save(output_file)?;
    Ok(())
}

/// Fills a quad region with its average color
fn fill_quad_with_color(output_image: &mut RgbaImage, leaf: &Quad) {
    let end_x = (leaf.x + leaf.width).min(output_image.width());
    let end_y = (leaf.y + leaf.height).min(output_image.height());
    
    for x in leaf.x..end_x {
        for y in leaf.y..end_y {
            output_image.put_pixel(x, y, leaf.color);
        }
    }
}

/// Draws the outline of a quad using line segments
fn draw_quad_outline(output_image: &mut RgbaImage, leaf: &Quad, color: Rgba<u8>) {
    let x1 = leaf.x as f32;
    let y1 = leaf.y as f32;
    let x2 = (leaf.x + leaf.width) as f32;
    let y2 = (leaf.y + leaf.height) as f32;
    
    // Top edge
    draw_line_segment_mut(output_image, (x1, y1), (x2, y1), color);
    // Bottom edge
    draw_line_segment_mut(output_image, (x1, y2), (x2, y2), color);
    // Left edge
    draw_line_segment_mut(output_image, (x1, y1), (x1, y2), color);
    // Right edge
    draw_line_segment_mut(output_image, (x2, y1), (x2, y2), color);
}

impl Quad {
    /// Creates a new Quad instance
    pub fn new(
        image: image::DynamicImage,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        config: QuadConfig,
    ) -> Quad {
        Quad {
            image: std::rc::Rc::new(image),
            x,
            y,
            width,
            height,
            color: Rgba([0, 0, 0, 255]),
            config,
            cur_depth: 0,
        }
    }

    /// Calculates the average color of all pixels within this quad
    pub fn calc_avg_color(&self) -> Rgba<u8> {
        let mut total_red: u64 = 0;
        let mut total_green: u64 = 0;
        let mut total_blue: u64 = 0;
        let mut pixel_count = 0;
        
        let end_x = (self.x + self.width).min(self.image.width());
        let end_y = (self.y + self.height).min(self.image.height());
        
        for x in self.x..end_x {
            for y in self.y..end_y {
                let pixel = self.image.get_pixel(x, y);
                let pixel_rgba = pixel.to_rgba();
                let rgba_arr = pixel_rgba.0;
                
                total_red += rgba_arr[0] as u64;
                total_green += rgba_arr[1] as u64;
                total_blue += rgba_arr[2] as u64;
                pixel_count += 1;
            }
        }
        
        if pixel_count == 0 {
            Rgba([0, 0, 0, 255])
        } else {
            let avg_red = (total_red / pixel_count) as u8;
            let avg_green = (total_green / pixel_count) as u8;
            let avg_blue = (total_blue / pixel_count) as u8;
            Rgba([avg_red, avg_green, avg_blue, 255])
        }
    }

    /// Calculates the color distance (variance) within this quad
    pub fn calc_color_distance(&self) -> f64 {
        let avg_color = self.calc_avg_color();
        let mut color_sum: f64 = 0.0;
        let mut pixel_count = 0;
        
        let end_x = (self.x + self.width).min(self.image.width());
        let end_y = (self.y + self.height).min(self.image.height());
        
        for x in self.x..end_x {
            for y in self.y..end_y {
                let pixel = self.image.get_pixel(x, y);
                let pixel_rgba = pixel.to_rgba();
                let rgba_arr = pixel_rgba.0;
                let avg_rgba = avg_color.0;

                // Calculate Euclidean distance in RGB space
                let r_diff = (avg_rgba[0] as f64 - rgba_arr[0] as f64).powi(2);
                let g_diff = (avg_rgba[1] as f64 - rgba_arr[1] as f64).powi(2);
                let b_diff = (avg_rgba[2] as f64 - rgba_arr[2] as f64).powi(2);
                
                color_sum += (r_diff + g_diff + b_diff).sqrt();
                pixel_count += 1;
            }
        }

        if pixel_count == 0 {
            0.0
        } else {
            color_sum / pixel_count as f64
        }
    }

    /// Subdivides this quad into 4 child quads
    pub fn subdivide(&self) -> [Quad; 4] {
        let new_width = (self.width + 1) / 2;  // Ceiling division
        let new_height = (self.height + 1) / 2;  // Ceiling division

        let x1 = self.x;
        let x2 = self.x + new_width;
        let y1 = self.y;
        let y2 = self.y + new_height;
        
        // Calculate remaining dimensions for bottom-right quad
        let remaining_width = self.width - new_width;
        let remaining_height = self.height - new_height;
        
        [
            // Top-left
            Quad {
                image: self.image.clone(),
                x: x1,
                y: y1,
                width: new_width,
                height: new_height,
                color: Rgba([0, 0, 0, 255]),
                config: self.config.clone(),
                cur_depth: self.cur_depth + 1,
            },
            // Top-right
            Quad {
                image: self.image.clone(),
                x: x2,
                y: y1,
                width: remaining_width,
                height: new_height,
                color: Rgba([0, 0, 0, 255]),
                config: self.config.clone(),
                cur_depth: self.cur_depth + 1,
            },
            // Bottom-left
            Quad {
                image: self.image.clone(),
                x: x1,
                y: y2,
                width: new_width,
                height: remaining_height,
                color: Rgba([0, 0, 0, 255]),
                config: self.config.clone(),
                cur_depth: self.cur_depth + 1,
            },
            // Bottom-right
            Quad {
                image: self.image.clone(),
                x: x2,
                y: y2,
                width: remaining_width,
                height: remaining_height,
                color: Rgba([0, 0, 0, 255]),
                config: self.config.clone(),
                cur_depth: self.cur_depth + 1,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    fn create_test_image() -> image::DynamicImage {
        let mut img = RgbaImage::new(100, 100);
        // Fill with a gradient pattern for testing
        for x in 0..100 {
            for y in 0..100 {
                let r = (x * 255 / 100) as u8;
                let g = (y * 255 / 100) as u8;
                let b = ((x + y) * 255 / 200) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
        image::DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn test_quad_creation() {
        let img = create_test_image();
        let config = QuadConfig::default();
        let quad = Quad::new(img, 0, 0, 50, 50, config);
        
        assert_eq!(quad.x, 0);
        assert_eq!(quad.y, 0);
        assert_eq!(quad.width, 50);
        assert_eq!(quad.height, 50);
        assert_eq!(quad.cur_depth, 0);
    }

    #[test]
    fn test_quad_subdivision() {
        let img = create_test_image();
        let config = QuadConfig::default();
        let quad = Quad::new(img, 0, 0, 100, 100, config);
        let children = quad.subdivide();
        
        assert_eq!(children.len(), 4);
        
        // Check top-left child
        assert_eq!(children[0].x, 0);
        assert_eq!(children[0].y, 0);
        assert_eq!(children[0].width, 50);
        assert_eq!(children[0].height, 50);
        assert_eq!(children[0].cur_depth, 1);
    }

    #[test]
    fn test_avg_color_calculation() {
        let mut img = RgbaImage::new(10, 10);
        // Fill with solid red
        for x in 0..10 {
            for y in 0..10 {
                img.put_pixel(x, y, Rgba([255, 0, 0, 255]));
            }
        }
        let dynamic_img = image::DynamicImage::ImageRgba8(img);
        let config = QuadConfig::default();
        let quad = Quad::new(dynamic_img, 0, 0, 10, 10, config);
        
        let avg_color = quad.calc_avg_color();
        assert_eq!(avg_color, Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_config_default() {
        let config = QuadConfig::default();
        assert_eq!(config.max_depth, 7);
        assert_eq!(config.color_threshold, 10.0);
        assert_eq!(config.size_threshold, 5);
        assert_eq!(config.output_file, "output.png");
    }
}