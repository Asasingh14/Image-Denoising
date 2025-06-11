use image::{GenericImageView, ImageBuffer, Rgb};
use rayon::prelude::*;
use std::path::Path;

/// Denoise an image using an improved Gaussian filter.
/// 
/// # Arguments
/// * `input_path` - Path to the input image
/// * `output_path` - Path to save the denoised image
/// * `kernel_radius` - Radius of the Gaussian kernel (u32, e.g. 1, 2, 3, ...)
/// * `sigma` - Standard deviation for the Gaussian kernel (f32)
/// 
/// Note: kernel_radius must be an integer (u32), not a float.
pub fn denoise_image_gaussian_improved(input_path: &str, output_path: &str, _kernel_radius: u32, sigma: f32) {
    let image = image::open(&Path::new(input_path)).unwrap();
    let (width, height) = image.dimensions();
    let image = image.to_rgb8();

    // Multi-pass filtering with decreasing sigma values
    let passes = 3;
    let mut current_image = image.clone();
    
    for pass in 0..passes {
        let pass_factor = 1.0 - (pass as f32 / passes as f32);
        let current_sigma = sigma * pass_factor;
        
        // Calculate kernel size based on sigma
        let kernel_size = (current_sigma * 6.0).ceil() as i32;
        let half_kernel = kernel_size / 2;
        
        // Generate Gaussian kernel
        let kernel = generate_gaussian_kernel(kernel_size, current_sigma);
        
        // Parallelize over rows
        let rows: Vec<Vec<u8>> = (0..height).into_par_iter().map(|y| {
            let mut row = Vec::with_capacity((width * 3) as usize);
            for x in 0..width {
                let center = current_image.get_pixel(x, y);
                let neighbors = get_neighbors(&current_image, x as i32, y as i32, half_kernel);
                
                // Convert center pixel to LAB
                let center_lab = rgb_to_lab(center[0], center[1], center[2]);
                
                // Calculate local statistics for adaptive parameters
                let (local_mean, local_std) = calculate_local_stats(&neighbors);
                let adaptive_sigma = current_sigma * (1.0 + local_std / 128.0);
                
                let new_color = gaussian_color_lab(
                    &neighbors,
                    center_lab,
                    x as i32,
                    y as i32,
                    &kernel,
                    adaptive_sigma,
                    local_mean
                );
                
                row.push(new_color[0]);
                row.push(new_color[1]);
                row.push(new_color[2]);
            }
            row
        }).collect();

        // Update current image for next pass
        let denoised_data: Vec<u8> = rows.into_iter().flatten().collect();
        current_image = ImageBuffer::from_vec(width, height, denoised_data).unwrap();
    }

    // Save the final result
    current_image.save(output_path).unwrap();
}

// Convert RGB to LAB color space
fn rgb_to_lab(r: u8, g: u8, b: u8) -> [f32; 3] {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let r = if r > 0.04045 { ((r + 0.055) / 1.055).powf(2.4) } else { r / 12.92 };
    let g = if g > 0.04045 { ((g + 0.055) / 1.055).powf(2.4) } else { g / 12.92 };
    let b = if b > 0.04045 { ((b + 0.055) / 1.055).powf(2.4) } else { b / 12.92 };

    let x = r * 0.4124 + g * 0.3576 + b * 0.1805;
    let y = r * 0.2126 + g * 0.7152 + b * 0.0722;
    let z = r * 0.0193 + g * 0.1192 + b * 0.9505;

    let x = x / 0.95047;
    let y = y / 1.0;
    let z = z / 1.08883;

    let x = if x > 0.008856 { x.powf(1.0/3.0) } else { (7.787 * x) + 16.0/116.0 };
    let y = if y > 0.008856 { y.powf(1.0/3.0) } else { (7.787 * y) + 16.0/116.0 };
    let z = if z > 0.008856 { z.powf(1.0/3.0) } else { (7.787 * z) + 16.0/116.0 };

    let l = (116.0 * y) - 16.0;
    let a = 500.0 * (x - y);
    let b = 200.0 * (y - z);

    [l, a, b]
}

// Convert LAB to RGB color space
fn lab_to_rgb(l: f32, a: f32, b: f32) -> [u8; 3] {
    let y = (l + 16.0) / 116.0;
    let x = a / 500.0 + y;
    let z = y - b / 200.0;

    let x = 0.95047 * (if x > 0.206893 { x.powi(3) } else { (x - 16.0/116.0) / 7.787 });
    let y = 1.0 * (if y > 0.206893 { y.powi(3) } else { (y - 16.0/116.0) / 7.787 });
    let z = 1.08883 * (if z > 0.206893 { z.powi(3) } else { (z - 16.0/116.0) / 7.787 });

    let r = x * 3.2406 + y * -1.5372 + z * -0.4986;
    let g = x * -0.9689 + y * 1.8758 + z * 0.0415;
    let b = x * 0.0557 + y * -0.2040 + z * 1.0570;

    let r = if r > 0.0031308 { 1.055 * r.powf(1.0/2.4) - 0.055 } else { 12.92 * r };
    let g = if g > 0.0031308 { 1.055 * g.powf(1.0/2.4) - 0.055 } else { 12.92 * g };
    let b = if b > 0.0031308 { 1.055 * b.powf(1.0/2.4) - 0.055 } else { 12.92 * b };

    [
        (r * 255.0).round().clamp(0.0, 255.0) as u8,
        (g * 255.0).round().clamp(0.0, 255.0) as u8,
        (b * 255.0).round().clamp(0.0, 255.0) as u8,
    ]
}

fn generate_gaussian_kernel(size: i32, sigma: f32) -> Vec<f32> {
    let mut kernel = Vec::with_capacity((size * size) as usize);
    let half_size = size / 2;
    let sigma_sq = 2.0 * sigma * sigma;
    let mut sum = 0.0;

    for y in -half_size..=half_size {
        for x in -half_size..=half_size {
            let dist = (x * x + y * y) as f32;
            let weight = (-dist / sigma_sq).exp();
            kernel.push(weight);
            sum += weight;
        }
    }

    // Normalize kernel
    for weight in &mut kernel {
        *weight /= sum;
    }

    kernel
}

fn calculate_local_stats(neighbors: &Vec<((i32, i32), [u8; 3])>) -> (f32, f32) {
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    let n = neighbors.len() as f32;

    for &(_, color) in neighbors {
        let intensity = (color[0] as f32 + color[1] as f32 + color[2] as f32) / 3.0;
        sum += intensity;
        sum_sq += intensity * intensity;
    }

    let mean = sum / n;
    let variance = (sum_sq / n) - (mean * mean);
    let std_dev = variance.sqrt();

    (mean, std_dev)
}

fn get_neighbors(image: &ImageBuffer<Rgb<u8>, Vec<u8>>, x: i32, y: i32, half_kernel: i32) -> Vec<((i32, i32), [u8; 3])> {
    let width = image.width() as i32;
    let height = image.height() as i32;
    let mut neighbors = Vec::new();
    
    // Use a circular window for better edge preservation
    for dy in -half_kernel..=half_kernel {
        for dx in -half_kernel..=half_kernel {
            let dist = (dx * dx + dy * dy) as f32;
            if dist <= (half_kernel * half_kernel) as f32 {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && ny >= 0 && nx < width && ny < height {
                    let pixel = image.get_pixel(nx as u32, ny as u32);
                    neighbors.push(((nx, ny), [pixel[0], pixel[1], pixel[2]]));
                }
            }
        }
    }
    neighbors
}

fn gaussian_color_lab(
    neighbors: &Vec<((i32, i32), [u8; 3])>,
    center_lab: [f32; 3],
    _x: i32,
    _y: i32,
    kernel: &Vec<f32>,
    sigma: f32,
    _local_mean: f32
) -> [u8; 3] {
    let mut weighted_sum = [0.0f32; 3];
    let mut total_weight = 0.0f32;
    let mut kernel_idx = 0;

    for &((_nx, _ny), color) in neighbors {
        let neighbor_lab = rgb_to_lab(color[0], color[1], color[2]);
        let color_diff = (
            (neighbor_lab[0] - center_lab[0]).powi(2) +
            (neighbor_lab[1] - center_lab[1]).powi(2) +
            (neighbor_lab[2] - center_lab[2]).powi(2)
        ).sqrt();

        let spatial_weight = kernel[kernel_idx];
        kernel_idx += 1;

        let range_weight = (-(color_diff * color_diff) / (2.0 * sigma * sigma)).exp();
        let weight = spatial_weight * range_weight;

        for c in 0..3 {
            weighted_sum[c] += neighbor_lab[c] * weight;
        }
        total_weight += weight;
    }

    if total_weight > 0.0 {
        let normalized_lab = [
            weighted_sum[0] / total_weight,
            weighted_sum[1] / total_weight,
            weighted_sum[2] / total_weight
        ];
        lab_to_rgb(normalized_lab[0], normalized_lab[1], normalized_lab[2])
    } else {
        lab_to_rgb(center_lab[0], center_lab[1], center_lab[2])
    }
} 