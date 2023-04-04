use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

pub fn denoise_image_imp(input_path: &str, output_path: &str, kernel_radius: f32, sigma: f32) {
    let image = image::open(&Path::new(input_path)).unwrap();
    let (width, height) = image.dimensions();

    let kernel = create_gaussian_kernel(kernel_radius, sigma);

    let denoised_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
        let neighbors = get_neighbors(&image, x as i32, y as i32, kernel_radius as i32);
        let weighted_color = apply_kernel(&neighbors, &kernel);
        Rgb([weighted_color[0], weighted_color[1], weighted_color[2]])
    });

    denoised_image.save(output_path).unwrap();
}

fn get_neighbors(image: &DynamicImage, x: i32, y: i32, radius: i32) -> Vec<(i32, i32, [u8; 3])> {
    let width = image.width() as i32;
    let height = image.height() as i32;
    let mut neighbors = Vec::new();

    for dy in -radius..=radius { //Make sure it gets like a circular area around the point x
        for dx in -radius..=radius { //Make sure it gets like a circular area around the point y
            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0 && ny >= 0 && nx < width && ny < height {
                let pixel = image.get_pixel(nx as u32, ny as u32);
                neighbors.push((dx, dy, [pixel[0], pixel[1], pixel[2]]));
            }
        }
    }

    neighbors
}

fn create_gaussian_kernel(radius: f32, sigma: f32) -> Vec<Vec<f32>> {
    let mut kernel = Vec::new();
    let mut sum = 0.0;
    let radius_i = radius.ceil() as i32;

    for y in -radius_i..=radius_i {
        let mut row = Vec::new();
        for x in -radius_i..=radius_i {

            //Formula to find gaussian weightage
            let gaussian = (-(x.pow(2) + y.pow(2)) as f32 / (2.0 * sigma.powi(2))).exp() / (2.0 * std::f32::consts::PI * sigma.powi(2));

            row.push(gaussian);
            sum += gaussian;
        }
        kernel.push(row);
    }

    for y in 0..kernel.len() {
        for x in 0..kernel[y].len() {
            kernel[y][x] /= sum;
        }
    }

    kernel
}


//Like average_color but using kernel(weightage)
fn apply_kernel(neighbors: &[(i32, i32, [u8; 3])], kernel: &[Vec<f32>]) -> [u8; 3] {
    let mut r_sum = 0.0;
    let mut g_sum = 0.0;
    let mut b_sum = 0.0;
    let kernel_radius = (kernel.len() / 2) as i32;

    for &(dx, dy, color) in neighbors {
        let kernel_value = kernel[(kernel_radius + dy) as usize][(kernel_radius + dx) as usize];
        r_sum += color[0] as f32 * kernel_value;
        g_sum += color[1] as f32 * kernel_value;
        b_sum += color[2] as f32 * kernel_value;
    }

    [
        r_sum.clamp(0.0, 255.0) as u8,
        g_sum.clamp(0.0, 255.0) as u8,
        b_sum.clamp(0.0, 255.0) as u8,
    ]
}

