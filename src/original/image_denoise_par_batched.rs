use image::{GenericImageView, ImageBuffer, Rgb};
use rayon::prelude::*;
use std::path::Path;

pub fn denoise_image_par_batched(input_path: &str, output_path: &str) {
    let image = image::open(&Path::new(input_path)).unwrap();
    let (width, height) = image.dimensions();
    let chunk_size = 64;

    // Prepare output buffer
    let mut denoised_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Calculate chunk grid
    let num_chunks_x = (width + chunk_size - 1) / chunk_size;
    let num_chunks_y = (height + chunk_size - 1) / chunk_size;

    // Prepare all chunk coordinates
    let chunk_coords: Vec<(u32, u32)> = (0..num_chunks_x)
        .flat_map(|cx| (0..num_chunks_y).map(move |cy| (cx, cy)))
        .collect();

    // For each chunk, process in parallel and collect results
    let chunk_results: Vec<((u32, u32), Vec<(u32, u32, [u8; 3])>)> = chunk_coords
        .into_par_iter()
        .map(|(chunk_x, chunk_y)| {
            let start_x = chunk_x * chunk_size;
            let start_y = chunk_y * chunk_size;
            let end_x = (start_x + chunk_size).min(width);
            let end_y = (start_y + chunk_size).min(height);
            let mut pixels = Vec::with_capacity(((end_x - start_x) * (end_y - start_y)) as usize);
            for y in start_y..end_y {
                for x in start_x..end_x {
                    let neighbors = get_neighbors(&image, x as i32, y as i32);
                    let avg_color = average_color(&neighbors);
                    pixels.push((x, y, avg_color));
                }
            }
            ((chunk_x, chunk_y), pixels)
        })
        .collect();

    // Write results back to the output image
    for (_chunk, pixels) in chunk_results {
        for (x, y, color) in pixels {
            denoised_image.put_pixel(x, y, Rgb([color[0], color[1], color[2]]));
        }
    }

    denoised_image.save(output_path).unwrap();
}

fn get_neighbors(image: &image::DynamicImage, x: i32, y: i32) -> Vec<[u8; 3]> {
    let width = image.width() as i32;
    let height = image.height() as i32;
    let dx = [-1, 0, 1, -1, 1, -1, 0, 1];
    let dy = [-1, -1, -1, 0, 0, 1, 1, 1];
    let mut neighbors = Vec::new();
    for i in 0..8 {
        let nx = x + dx[i];
        let ny = y + dy[i];
        if nx >= 0 && ny >= 0 && nx < width && ny < height {
            let pixel = image.get_pixel(nx as u32, ny as u32);
            neighbors.push([pixel[0], pixel[1], pixel[2]]);
        }
    }
    neighbors
}

fn average_color(neighbors: &Vec<[u8; 3]>) -> [u8; 3] {
    let count = neighbors.len() as u32;
    let mut r_sum = 0;
    let mut g_sum = 0;
    let mut b_sum = 0;
    for color in neighbors {
        r_sum += color[0] as u32;
        g_sum += color[1] as u32;
        b_sum += color[2] as u32;
    }
    [
        (r_sum / count) as u8,
        (g_sum / count) as u8,
        (b_sum / count) as u8,
    ]
} 