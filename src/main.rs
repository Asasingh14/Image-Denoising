use std::time::Instant;

use crate::img_denoise::*;
use crate::img_denoise_imp::*;
use crate::img_denoise_par::*;

mod img_denoise;
mod img_denoise_imp;
mod img_denoise_par;

fn main() {
    println!("Hello, world!");

    let mut start = Instant::now();
    test_img_denoise(); //Basic Denoise
    println!("Basic Denoise time: {:?}", Instant::now() - start);

    let mut start = Instant::now();
    test_img_denoise_par();//Basic Denoise Parallel
    println!("Basic Denoise parallel time: {:?}", Instant::now() - start);

    let mut start = Instant::now();
    test_img_denoise_imp(); //Gaussian Denoise
    println!("Gaussian Denoise time: {:?}", Instant::now() - start);
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

fn test_img_denoise_imp() {
    let input_file_1 = "test_file/image1.jpeg";
    let output_file_1 = "res_dump/out1_imp.jpeg";
    denoise_image_imp(input_file_1, output_file_1, 1.5, 1.0);

    let input_file_2 = "test_file/image2.jpg";
    let output_file_2 = "res_dump/out2_imp.jpeg";
    denoise_image_imp(input_file_2, output_file_2, 1.5, 1.0);

    let input_file_3 = "test_file/image3.jpg";
    let output_file_3 = "res_dump/out3_imp.jpeg";
    denoise_image_imp(input_file_3, output_file_3, 1.5, 1.0);

    let input_file_4 = "test_file/image4.jpg";
    let output_file_4 = "res_dump/out4_imp.jpeg";
    denoise_image_imp(input_file_4, output_file_4, 1.5, 1.0);

    let input_file_5 = "test_file/image5.jpg";
    let output_file_5 = "res_dump/out5_imp.jpeg";
    denoise_image_imp(input_file_5, output_file_5, 1.5, 1.0);

    let input_file_6 = "test_file/image6.jpg";
    let output_file_6 = "res_dump/out6_imp.jpeg";
    denoise_image_imp(input_file_6, output_file_6, 1.5, 1.0);
}