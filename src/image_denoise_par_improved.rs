use image::{GenericImageView, ImageBuffer, Rgb};
use rayon::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn denoise_image_par_imp(input_path: &str, output_path: &str) {//Got help from CHAT-GPT and read alot of stackoverflow
    // Open image and get pixel data, width and height vals
    let image = image::open(&Path::new(input_path)).unwrap();
    // Get dimensions of image
    let (width, height) = image.dimensions();

    // Create a new image buffer with the same dimensions as the input image
    let denoised_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Wrap the denoised_image in an Arc<Mutex<...>> to allow safe sharing across threads
    let denoised_image = Arc::new(Mutex::new(denoised_image));

    // Define chunk size
    let chunk_size = 64;

    // Calculate the number of chunks for width and height
    let num_chunks_width = (width + chunk_size - 1) / chunk_size;
    let num_chunks_height = (height + chunk_size - 1) / chunk_size;

    // Create a range for chunk indices
    let chunk_indices: Vec<(u32, u32)> = (0..num_chunks_height)
        .flat_map(move |y| (0..num_chunks_width).map(move |x| (x, y)))
        .collect();

    chunk_indices.par_iter().for_each(|(chunk_x, chunk_y)| {
        let chunk_start_x = chunk_x * chunk_size;
        let chunk_start_y = chunk_y * chunk_size;
        let chunk_end_x = (chunk_start_x + chunk_size).min(width);
        let chunk_end_y = (chunk_start_y + chunk_size).min(height);

        for y in chunk_start_y..chunk_end_y {
            for x in chunk_start_x..chunk_end_x {
                let neighbors = get_neighbors(&image, x as i32, y as i32);
                let avg_color = average_color(&neighbors);

                // Lock the denoised_image to safely access and modify it
                let mut denoised_image = denoised_image.lock().unwrap();
                denoised_image.put_pixel(x, y, Rgb([avg_color[0], avg_color[1], avg_color[2]]));
            }
        }
    });

    // Save the denoised image
    Arc::try_unwrap(denoised_image)
        .unwrap_or_else(|_| panic!("Failed to unwrap Arc"))
        .into_inner()
        .unwrap_or_else(|_| panic!("Failed to unwrap Mutex"))
        .save(output_path)
        .unwrap();
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