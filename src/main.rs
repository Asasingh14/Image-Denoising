use std::{fs, io};
use std::time::Instant;

use crate::original::img_denoise::*;

mod original {
    pub mod img_denoise;
    pub mod img_denoise_gaussian;
    pub mod img_denoise_par;
    pub mod image_denoise_par_improved;
    pub mod image_denoise_gaussian_par;
    pub mod image_denoise_par_batched;
}

mod improved {
    pub mod img_denoise_improved;
    pub mod img_denoise_bilateral;
    pub mod img_denoise_gaussian_improved;
}

use original::img_denoise_gaussian::denoise_image_gaussian as denoise_image_gaussian_original;
use original::img_denoise_par::denoise_image_par;
use original::image_denoise_par_improved::denoise_image_par_imp;
use original::image_denoise_gaussian_par::denoise_image_gaussian_par;
use original::image_denoise_par_batched::denoise_image_par_batched;

use improved::img_denoise_bilateral::denoise_image_bilateral;
use improved::img_denoise_improved::denoise_image_improved;
use improved::img_denoise_gaussian_improved::denoise_image_gaussian_improved;

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

    let start = Instant::now();
    test_img_denoise_par_batched();
    println!("Basic Denoise parallel batched time: {:?}", Instant::now() - start);

    let start = Instant::now();
    test_img_denoise_improved();
    println!("Improved Denoise time: {:?}", Instant::now() - start);

    let start = Instant::now();
    test_img_denoise_bilateral();
    println!("Bilateral Denoise time: {:?}", Instant::now() - start);

    let start = Instant::now();
    test_img_denoise_gaussian_improved();
    println!("Gaussian Improved Denoise time: {:?}", Instant::now() - start);
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
    denoise_image_gaussian_original(input_file_1, output_file_1, 1.5, 1.0);

    let input_file_2 = "test_file/image2.jpg";
    let output_file_2 = "res_dump/out2_gaussian.jpeg";
    denoise_image_gaussian_original(input_file_2, output_file_2, 1.5, 1.0);

    let input_file_3 = "test_file/image3.jpg";
    let output_file_3 = "res_dump/out3_gaussian.jpeg";
    denoise_image_gaussian_original(input_file_3, output_file_3, 1.5, 1.0);

    let input_file_4 = "test_file/image4.jpg";
    let output_file_4 = "res_dump/out4_gaussian.jpeg";
    denoise_image_gaussian_original(input_file_4, output_file_4, 1.5, 1.0);

    let input_file_5 = "test_file/image5.jpg";
    let output_file_5 = "res_dump/out5_gaussian.jpeg";
    denoise_image_gaussian_original(input_file_5, output_file_5, 1.5, 1.0);

    let input_file_6 = "test_file/image6.jpg";
    let output_file_6 = "res_dump/out6_gaussian.jpeg";
    denoise_image_gaussian_original(input_file_6, output_file_6, 1.5, 1.0);
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

fn test_img_denoise_par_batched() {
    let input_file_1 = "test_file/image1.jpeg";
    let output_file_1 = "res_dump/out1_par_batched.jpeg";
    denoise_image_par_batched(input_file_1, output_file_1);

    let input_file_2 = "test_file/image2.jpg";
    let output_file_2 = "res_dump/out2_par_batched.jpeg";
    denoise_image_par_batched(input_file_2, output_file_2);

    let input_file_3 = "test_file/image3.jpg";
    let output_file_3 = "res_dump/out3_par_batched.jpeg";
    denoise_image_par_batched(input_file_3, output_file_3);

    let input_file_4 = "test_file/image4.jpg";
    let output_file_4 = "res_dump/out4_par_batched.jpeg";
    denoise_image_par_batched(input_file_4, output_file_4);

    let input_file_5 = "test_file/image5.jpg";
    let output_file_5 = "res_dump/out5_par_batched.jpeg";
    denoise_image_par_batched(input_file_5, output_file_5);

    let input_file_6 = "test_file/image6.jpg";
    let output_file_6 = "res_dump/out6_par_batched.jpeg";
    denoise_image_par_batched(input_file_6, output_file_6);
}

fn test_img_denoise_improved() {
    let input_file_1 = "test_file/image1.jpeg";
    let output_file_1 = "res_dump/out1_improved.jpeg";
    denoise_image_improved(input_file_1, output_file_1, 3);

    let input_file_2 = "test_file/image2.jpg";
    let output_file_2 = "res_dump/out2_improved.jpeg";
    denoise_image_improved(input_file_2, output_file_2, 3);

    let input_file_3 = "test_file/image3.jpg";
    let output_file_3 = "res_dump/out3_improved.jpeg";
    denoise_image_improved(input_file_3, output_file_3, 3);

    let input_file_4 = "test_file/image4.jpg";
    let output_file_4 = "res_dump/out4_improved.jpeg";
    denoise_image_improved(input_file_4, output_file_4, 3);

    let input_file_5 = "test_file/image5.jpg";
    let output_file_5 = "res_dump/out5_improved.jpeg";
    denoise_image_improved(input_file_5, output_file_5, 3);

    let input_file_6 = "test_file/image6.jpg";
    let output_file_6 = "res_dump/out6_improved.jpeg";
    denoise_image_improved(input_file_6, output_file_6, 3);
}

fn test_img_denoise_bilateral() {
    let input_file_1 = "test_file/image1.jpeg";
    let output_file_1 = "res_dump/out1_bilateral.jpeg";
    denoise_image_bilateral(input_file_1, output_file_1, 5, 3.0, 30.0);

    let input_file_2 = "test_file/image2.jpg";
    let output_file_2 = "res_dump/out2_bilateral.jpeg";
    denoise_image_bilateral(input_file_2, output_file_2, 5, 3.0, 30.0);

    let input_file_3 = "test_file/image3.jpg";
    let output_file_3 = "res_dump/out3_bilateral.jpeg";
    denoise_image_bilateral(input_file_3, output_file_3, 5, 3.0, 30.0);

    let input_file_4 = "test_file/image4.jpg";
    let output_file_4 = "res_dump/out4_bilateral.jpeg";
    denoise_image_bilateral(input_file_4, output_file_4, 5, 3.0, 30.0);

    let input_file_5 = "test_file/image5.jpg";
    let output_file_5 = "res_dump/out5_bilateral.jpeg";
    denoise_image_bilateral(input_file_5, output_file_5, 5, 3.0, 30.0);

    let input_file_6 = "test_file/image6.jpg";
    let output_file_6 = "res_dump/out6_bilateral.jpeg";
    denoise_image_bilateral(input_file_6, output_file_6, 5, 3.0, 30.0);
}

fn test_img_denoise_gaussian_improved() {
    let input_file_1 = "test_file/image1.jpeg";
    let output_file_1 = "res_dump/out1_gaussian_improved.jpeg";
    denoise_image_gaussian_improved(input_file_1, output_file_1, 2, 2.0);

    let input_file_2 = "test_file/image2.jpg";
    let output_file_2 = "res_dump/out2_gaussian_improved.jpeg";
    denoise_image_gaussian_improved(input_file_2, output_file_2, 2, 2.0);

    let input_file_3 = "test_file/image3.jpg";
    let output_file_3 = "res_dump/out3_gaussian_improved.jpeg";
    denoise_image_gaussian_improved(input_file_3, output_file_3, 2, 2.0);

    let input_file_4 = "test_file/image4.jpg";
    let output_file_4 = "res_dump/out4_gaussian_improved.jpeg";
    denoise_image_gaussian_improved(input_file_4, output_file_4, 2, 2.0);

    let input_file_5 = "test_file/image5.jpg";
    let output_file_5 = "res_dump/out5_gaussian_improved.jpeg";
    denoise_image_gaussian_improved(input_file_5, output_file_5, 2, 2.0);

    let input_file_6 = "test_file/image6.jpg";
    let output_file_6 = "res_dump/out6_gaussian_improved.jpeg";
    denoise_image_gaussian_improved(input_file_6, output_file_6, 2, 2.0);
}