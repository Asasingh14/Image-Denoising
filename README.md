# funpar-t2-22-project-name_here
funpar-t2-22-project-name_here created by GitHub Classroom

# Image Denoising Project

## 2 techniques are used

1.0) Basic Denoise (denoise_image)

This function implements a basic denoising algorithm that averages the color values of a pixel's neighbors to reduce noise.

1.1) Basic Denoise in Parallel (denoise_image_par)

This function implements a basic denoising algorithm that averages the color values of a pixel's neighbors to reduce noise in parallel

1.3) Improved Basic Denoise in Parallel (denoise_image_par_imp)

An improved version of the denoise_image_par() function

2.0) Gaussian Denoise (denoise_image_gaussian)

This function implements an improved denoising algorithm using a Gaussian blur technique. <br>
It generates a Gaussian kernel matrix and applies it to each pixel and its neighbors in the input image to reduce noise.

2.1) Gaussian Denoise (denoise_image_gaussian_par)<br>
Same as denoise_image_gaussian() function but in parallel

## Basic Info

The code is in found src. <br><br>
Noisy images are found in test_file directory. <br><br>
Processed images are found in res_dump with names <b>xx.jpeg, xx_par.jpeg, xx_par_imp.jpeg, xx_gaussian.jpeg or xx_gaussian_par.jpeg </b> depending on the function used to denoise it. <br><br>
AI denoised images have also been included to show some baseline for the performance of denoising algorithms. <br>

## Conclusion from project

These denoising functions provide a simple and efficient way to reduce noise in images. <br>
The basic denoising function (denoise_image) offers a simple averaging approach and is fast enough for smaller batches.<br>
The basic denoising function in parallel (denoise_image_par) offers the same as above but we do not see the full benefit of parallel computing as the workload is not large enough.<br>
The improved denoising function (denoise_image_imp) utilizes Gaussian blur for better noise reduction and is blaZingly fast for the quality if offers.<br>
