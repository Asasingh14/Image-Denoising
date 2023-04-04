use std::fs::File;
use std::path::Path;

use image::{GenericImageView, ImageBuffer, Rgb};
use image::codecs::jpeg::JpegEncoder;
use rayon::prelude::*;

pub fn denoise_image_par(input_path: &str, output_path: &str) {

    //Open image and get pixel data, width and height vals
    let image = image::open(&Path::new(input_path)).unwrap();
    // Get dimensions of image
    let (width, height) = image.dimensions();

    // Create a new image buffer with the same dimensions as the input image
    let mut denoised_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Use Rayon's parallel iterator to process each pixel in parallel
    denoised_image
        .enumerate_pixels_mut()
        .par_bridge() // As image buffer cannot use par_iter() so hv to use enumerate_pixels_mut() and par_bridge()
        .for_each(|(x, y, pixel)| {
            // Get neighbouring pixels for each pixel
            let neighbors = get_neighbors(&image, x as i32, y as i32);
            //Get average color of neighbouring pixels
            let avg_color = average_color(&neighbors);
            //Return a pixel with avg color
            *pixel = Rgb([avg_color[0], avg_color[1], avg_color[2]]);
        });

    denoised_image.save(output_path).unwrap();
}

fn get_neighbors(image: &image::DynamicImage, x: i32, y: i32) -> Vec<[u8; 3]> {
    let width = image.width() as i32;
    let height = image.height() as i32;

    /*
    Get offsets for the diff pixels around the target pixels
      -1 -1  |  0 -1  |  1 -1
      -1  0  |  0  0  |  1  0
      -1  1  |  0  1  |  1  1
     */
    let dx = [-1, 0, 1, -1, 1, -1, 0, 1];
    let dy = [-1, -1, -1, 0, 0, 1, 1, 1];


    //Process neighbouring pixel offsets in parallel
    dx.par_iter()
        .zip(dy.par_iter())
        .filter_map(|(&dx, &dy)| {
            let nx = x + dx; //Get x coord nbr
            let ny = y + dy; //Get y coord nbr

            if nx >= 0 && ny >= 0 && nx < width && ny < height { //Check if x and y nbr exists
                let pixel = image.get_pixel(nx as u32, ny as u32);
                Some([pixel[0], pixel[1], pixel[2]])
            } else {
                None
            }
        })
        .collect::<Vec<[u8; 3]>>()
}

fn average_color(neighbors: &Vec<[u8; 3]>) -> [u8; 3] {
    let count = neighbors.len() as u32;
    let mut r_sum = 0;
    let mut g_sum = 0;
    let mut b_sum = 0;

    //Sum the red, green, and blue components of all colors
    for color in neighbors { //Used seq as data race can occur
        r_sum += color[0] as u32;
        g_sum += color[1] as u32;
        b_sum += color[2] as u32;
    }

    // Calculate the average of red, green, and blue components and return the result as a color
    [
        (r_sum / count) as u8,
        (g_sum / count) as u8,
        (b_sum / count) as u8,
    ]
}
