use std::{fs, io};
use std::time::Instant;

use crate::img_denoise::*;
use crate::img_denoise_gaussian::*;
use crate::img_denoise_par::*;
use crate::image_denoise_par_improved::*;
use crate::image_denoise_gaussian_par::*;

mod img_denoise;
mod img_denoise_gaussian;
mod img_denoise_par;
mod image_denoise_par_improved;
mod image_denoise_gaussian_par;

fn main() {
    println!("Hello, world!");
    println!("Funpar Project: Image Denoising");

    println!("Wiping Folder: \"res_dump\"");
    match wipe_folder("res_dump") {
        Ok(_) => println!("Successfully wiped the folder clean."),
        Err(e) => println!("Error wiping the folder: {}", e),
    }

    println!("Times: ");

    let start = Instant::now();
    test_img_denoise(); //Basic Denoise
    println!("Basic Denoise time: {:?}", Instant::now() - start);

    let start = Instant::now();
    test_img_denoise_par();//Basic Denoise Parallel
    println!("Basic Denoise parallel time: {:?}", Instant::now() - start);

    let start = Instant::now();
    test_img_denoise_par_imp();//Improved Basic Denoise Parallel
    println!("Basic Denoise improved parallel time: {:?}", Instant::now() - start);

    let start = Instant::now();
    test_img_denoise_gaussian(); //Gaussian Denoise
    println!("Gaussian Denoise time: {:?}", Instant::now() - start);

    let start = Instant::now();
    test_img_denoise_gaussian_par(); //Parallel Gaussian Denoise
    println!("Gaussian Denoise parallel time: {:?}", Instant::now() - start);
}

fn wipe_folder(folder_path: &str) -> io::Result<()> {
    // Recursively remove all files and subdirectories within the folder
    fs::remove_dir_all(folder_path)?;
    // Recreate the empty folder
    fs::create_dir(folder_path)?;
    Ok(())
}

fn test_img_denoise() {
    let input_file_1 = "test_file/image1.jpeg";
    let output_file_1 = "res_dump/out1.jpeg";
    denoise_image(input_file_1, output_file_1);

    let input_file_2 = "test_file/image2.jpg";
    let output_file_2 = "res_dump/out2.jpeg";
    denoise_image(input_file_2, output_file_2);

    let input_file_3 = "test_file/image3.jpg";
    let output_file_3 = "res_dump/out3.jpeg";
    denoise_image(input_file_3, output_file_3);

    let input_file_4 = "test_file/image4.jpg";
    let output_file_4 = "res_dump/out4.jpeg";
    denoise_image(input_file_4, output_file_4);

    let input_file_5 = "test_file/image5.jpg";
    let output_file_5 = "res_dump/out5.jpeg";
    denoise_image(input_file_5, output_file_5);

    let input_file_6 = "test_file/image6.jpg";
    let output_file_6 = "res_dump/out6.jpeg";
    denoise_image(input_file_6, output_file_6);
}

fn test_img_denoise_par() {
    let input_file_1 = "test_file/image1.jpeg";
    let output_file_1 = "res_dump/out1_par.jpeg";
    denoise_image_par(input_file_1, output_file_1);

    let input_file_2 = "test_file/image2.jpg";
    let output_file_2 = "res_dump/out2_par.jpeg";
    denoise_image_par(input_file_2, output_file_2);

    let input_file_3 = "test_file/image3.jpg";
    let output_file_3 = "res_dump/out3_par.jpeg";
    denoise_image_par(input_file_3, output_file_3);

    let input_file_4 = "test_file/image4.jpg";
    let output_file_4 = "res_dump/out4_par.jpeg";
    denoise_image_par(input_file_4, output_file_4);

    let input_file_5 = "test_file/image5.jpg";
    let output_file_5 = "res_dump/out5_par.jpeg";
    denoise_image_par(input_file_5, output_file_5);

    let input_file_6 = "test_file/image6.jpg";
    let output_file_6 = "res_dump/out6_par.jpeg";
    denoise_image_par(input_file_6, output_file_6);
}

fn test_img_denoise_par_imp() {
    let input_file_1 = "test_file/image1.jpeg";
    let output_file_1 = "res_dump/out1_par_imp.jpeg";
    denoise_image_par_imp(input_file_1, output_file_1);

    let input_file_2 = "test_file/image2.jpg";
    let output_file_2 = "res_dump/out2_par_imp.jpeg";
    denoise_image_par_imp(input_file_2, output_file_2);

    let input_file_3 = "test_file/image3.jpg";
    let output_file_3 = "res_dump/out3_par_imp.jpeg";
    denoise_image_par_imp(input_file_3, output_file_3);

    let input_file_4 = "test_file/image4.jpg";
    let output_file_4 = "res_dump/out4_par_imp.jpeg";
    denoise_image_par_imp(input_file_4, output_file_4);

    let input_file_5 = "test_file/image5.jpg";
    let output_file_5 = "res_dump/out5_par_imp.jpeg";
    denoise_image_par_imp(input_file_5, output_file_5);

    let input_file_6 = "test_file/image6.jpg";
    let output_file_6 = "res_dump/out6_par_imp.jpeg";
    denoise_image_par_imp(input_file_6, output_file_6);
}

fn test_img_denoise_gaussian() {
    let input_file_1 = "test_file/image1.jpeg";
    let output_file_1 = "res_dump/out1_gaussian.jpeg";
    denoise_image_gaussian(input_file_1, output_file_1, 1.5, 1.0);

    let input_file_2 = "test_file/image2.jpg";
    let output_file_2 = "res_dump/out2_gaussian.jpeg";
    denoise_image_gaussian(input_file_2, output_file_2, 1.5, 1.0);

    let input_file_3 = "test_file/image3.jpg";
    let output_file_3 = "res_dump/out3_gaussian.jpeg";
    denoise_image_gaussian(input_file_3, output_file_3, 1.5, 1.0);

    let input_file_4 = "test_file/image4.jpg";
    let output_file_4 = "res_dump/out4_gaussian.jpeg";
    denoise_image_gaussian(input_file_4, output_file_4, 1.5, 1.0);

    let input_file_5 = "test_file/image5.jpg";
    let output_file_5 = "res_dump/out5_gaussian.jpeg";
    denoise_image_gaussian(input_file_5, output_file_5, 1.5, 1.0);

    let input_file_6 = "test_file/image6.jpg";
    let output_file_6 = "res_dump/out6_gaussian.jpeg";
    denoise_image_gaussian(input_file_6, output_file_6, 1.5, 1.0);
}

fn test_img_denoise_gaussian_par() {
    let input_file_1 = "test_file/image1.jpeg";
    let output_file_1 = "res_dump/out1_gaussian_par.jpeg";
    denoise_image_gaussian_par(input_file_1, output_file_1, 1.5, 1.0);

    let input_file_2 = "test_file/image2.jpg";
    let output_file_2 = "res_dump/out2_gaussian_par.jpeg";
    denoise_image_gaussian_par(input_file_2, output_file_2, 1.5, 1.0);

    let input_file_3 = "test_file/image3.jpg";
    let output_file_3 = "res_dump/out3_gaussian_par.jpeg";
    denoise_image_gaussian_par(input_file_3, output_file_3, 1.5, 1.0);

    let input_file_4 = "test_file/image4.jpg";
    let output_file_4 = "res_dump/out4_gaussian_par.jpeg";
    denoise_image_gaussian_par(input_file_4, output_file_4, 1.5, 1.0);

    let input_file_5 = "test_file/image5.jpg";
    let output_file_5 = "res_dump/out5_gaussian_par.jpeg";
    denoise_image_gaussian_par(input_file_5, output_file_5, 1.5, 1.0);

    let input_file_6 = "test_file/image6.jpg";
    let output_file_6 = "res_dump/out6_gaussian_par.jpeg";
    denoise_image_gaussian_par(input_file_6, output_file_6, 1.5, 1.0);
}