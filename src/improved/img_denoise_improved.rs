use image::{GenericImageView, ImageBuffer, Rgb};
use rayon::prelude::*;
use std::path::Path;

pub fn denoise_image_improved(input_path: &str, output_path: &str, window_size: u32) {
    let image = image::open(&Path::new(input_path)).unwrap();
    let (width, height) = image.dimensions();
    let image = image.to_rgb8();
    let half_window = window_size as i32 / 2;

    // Parallelize over rows, each thread returns a Vec<u8> for its row
    let rows: Vec<Vec<u8>> = (0..height).into_par_iter().map(|y| {
        let mut row = Vec::with_capacity((width * 3) as usize);
        for x in 0..width {
            let neighbors = get_neighbors(&image, x as i32, y as i32, half_window);
            let median_color = median_color(&neighbors);
            row.push(median_color[0]);
            row.push(median_color[1]);
            row.push(median_color[2]);
        }
        row
    }).collect();

    // Flatten rows into a single buffer
    let denoised_data: Vec<u8> = rows.into_iter().flatten().collect();
    let denoised_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_vec(width, height, denoised_data).unwrap();
    denoised_image.save(output_path).unwrap();
}

fn get_neighbors(image: &ImageBuffer<Rgb<u8>, Vec<u8>>, x: i32, y: i32, half_window: i32) -> Vec<[u8; 3]> {
    let width = image.width() as i32;
    let height = image.height() as i32;
    let mut neighbors = Vec::new();
    for dy in -half_window..=half_window {
        for dx in -half_window..=half_window {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && ny >= 0 && nx < width && ny < height {
                let pixel = image.get_pixel(nx as u32, ny as u32);
                neighbors.push([pixel[0], pixel[1], pixel[2]]);
            }
        }
    }
    neighbors
}

fn median_color(neighbors: &Vec<[u8; 3]>) -> [u8; 3] {
    let mut r_values: Vec<u8> = neighbors.iter().map(|color| color[0]).collect();
    let mut g_values: Vec<u8> = neighbors.iter().map(|color| color[1]).collect();
    let mut b_values: Vec<u8> = neighbors.iter().map(|color| color[2]).collect();
    r_values.sort();
    g_values.sort();
    b_values.sort();
    let mid = r_values.len() / 2;
    [r_values[mid], g_values[mid], b_values[mid]]
} 