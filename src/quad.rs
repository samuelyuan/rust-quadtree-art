use image::{GenericImageView, Pixel, Rgba, RgbaImage};
use imageproc::drawing::draw_line_segment_mut;
use std::collections::VecDeque;
use std::fmt;
use std::vec::Vec;

#[derive(Clone)]
pub struct Quad {
    image: image::DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: Rgba<u8>,
    max_depth: u32,
    cur_depth: u32,
}

impl fmt::Display for Quad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(<{}, {}> {} x {}, cur depth: {}, max depth: {})",
            self.x, self.y, self.width, self.height, self.cur_depth, self.max_depth
        )
    }
}

pub fn subdivide_nodes(initial_quad: Quad) -> Vec<Quad> {
    let mut deque: VecDeque<Quad> = VecDeque::new();
    deque.push_back(initial_quad.clone());
    let mut quadtree_leaves: Vec<Quad> = Vec::new();
    let color_distance_threshold = 10.0;
    let image_dimension_threshold = 5;

    while !deque.is_empty() {
        let mut next_quad = deque.pop_front().expect("Cannot dequeue from empty queue.");

        println!("Next quad: {}", next_quad);
        if next_quad.x >= initial_quad.width {
            continue;
        }

        if next_quad.cur_depth < next_quad.max_depth
            && next_quad.calc_avg_color_distance() > color_distance_threshold
            && next_quad.width > image_dimension_threshold
            && next_quad.height > image_dimension_threshold
        {
            let children = next_quad.subdivide();
            for i in 0..4 {
                let child_node = children[i].clone();
                deque.push_back(child_node);
            }
        } else {
            quadtree_leaves.push(next_quad);
        }
    }
    return quadtree_leaves;
}

pub fn generate_image(quadtree_leaves: Vec<Quad>, image_width: u32, image_height: u32) {
    let mut output_image = RgbaImage::new(image_width, image_height);
    let black = Rgba([0, 0, 0, 255]);

    for leaf in quadtree_leaves {
        for x in (leaf.x)..(leaf.x + leaf.width) {
            for y in (leaf.y)..(leaf.y + leaf.height) {
                if x >= output_image.width() || y >= output_image.height() {
                    continue;
                }
                output_image.put_pixel(x, y, leaf.color);
            }
        }
        // Draw outline around leaf
        draw_line_segment_mut(
            &mut output_image,
            (leaf.x as f32, leaf.y as f32),
            ((leaf.x + leaf.width) as f32, leaf.y as f32),
            black,
        );
        draw_line_segment_mut(
            &mut output_image,
            (leaf.x as f32, (leaf.y + leaf.height) as f32),
            ((leaf.x + leaf.width) as f32, (leaf.y + leaf.height) as f32),
            black,
        );
        draw_line_segment_mut(
            &mut output_image,
            (leaf.x as f32, leaf.y as f32),
            (leaf.x as f32, (leaf.y + leaf.height) as f32),
            black,
        );
        draw_line_segment_mut(
            &mut output_image,
            ((leaf.x + leaf.width) as f32, leaf.y as f32),
            ((leaf.x + leaf.width) as f32, (leaf.y + leaf.height) as f32),
            black,
        );
    }
    output_image.save("output.png").unwrap();
}

impl Quad {
    pub fn new(
        image: image::DynamicImage,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        max_depth: u32,
        cur_depth: u32,
    ) -> Quad {
        Quad {
            image: image,
            x: x,
            y: y,
            width: width,
            height: height,
            color: Rgba([0, 0, 0, 255]),
            cur_depth: cur_depth,
            max_depth: max_depth,
        }
    }

    pub fn calc_avg_color_distance(&self) -> f64 {
        let mut color_sum: f64 = 0.0;
        for x in (self.x)..(self.x + self.width) {
            for y in (self.y)..(self.y + self.height) {
                if x >= self.image.width() || y >= self.image.height() {
                    continue;
                }
                let pixel = self.image.get_pixel(x, y);
                let pixel_rgba = pixel.to_rgba();
                let rgba_arr = pixel_rgba.0;

                let avg_color_rgba = self.color.to_rgba().0;
                color_sum += (avg_color_rgba[0] as f64 - rgba_arr[0] as f64).abs(); // red
                color_sum += (avg_color_rgba[1] as f64 - rgba_arr[1] as f64).abs(); // green
                color_sum += (avg_color_rgba[2] as f64 - rgba_arr[2] as f64).abs(); // blue
            }
        }

        return (color_sum as f64) / (3.0 * self.width as f64 * self.height as f64);
    }

    pub fn calc_avg_color(&self) -> Rgba<u8> {
        let mut total_red: u32 = 0;
        let mut total_green: u32 = 0;
        let mut total_blue: u32 = 0;
        for x in (self.x)..(self.x + self.width) {
            for y in (self.y)..(self.y + self.height) {
                if x >= self.image.width() || y >= self.image.height() {
                    continue;
                }
                let pixel = self.image.get_pixel(x, y);
                let pixel_rgba = pixel.to_rgba();
                let rgba_arr = pixel_rgba.0;
                total_red += rgba_arr[0] as u32;
                total_green += rgba_arr[1] as u32;
                total_blue += rgba_arr[2] as u32;
            }
        }
        let area: f64 = (self.width * self.height).into();
        let avg_red = total_red as f64 / area;
        let avg_green = total_green as f64 / area;
        let avg_blue = total_blue as f64 / area;
        return Rgba([avg_red as u8, avg_green as u8, avg_blue as u8, 255]);
    }

    pub fn subdivide(&self) -> [Quad; 4] {
        let new_width: u32 = (self.width as f64 / 2.0).ceil() as u32;
        let new_height: u32 = (self.height as f64 / 2.0).ceil() as u32;

        let x1 = self.x;
        let x2 = self.x + new_width;
        let y1 = self.y;
        let y2 = self.y + new_height;
        return [
            Quad::new(
                self.image.clone(),
                x1,
                y1,
                new_width,
                new_height,
                self.max_depth,
                self.cur_depth + 1,
            ),
            Quad::new(
                self.image.clone(),
                x2,
                y1,
                new_width,
                new_height,
                self.max_depth,
                self.cur_depth + 1,
            ),
            Quad::new(
                self.image.clone(),
                x1,
                y2,
                new_width,
                new_height,
                self.max_depth,
                self.cur_depth + 1,
            ),
            Quad::new(
                self.image.clone(),
                x2,
                y2,
                new_width,
                new_height,
                self.max_depth,
                self.cur_depth + 1,
            ),
        ];
    }
}
