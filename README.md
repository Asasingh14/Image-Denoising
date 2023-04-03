# funpar-t2-22-project-name_here
funpar-t2-22-project-name_here created by GitHub Classroom

# Image Denoising Project

## 2 techniques are used

1.0) Basic Denoise (denoise_image)

This function implements a basic denoising algorithm that averages the color values of a pixel's neighbors to reduce noise.



1.1) Basic Denoise in Parallel (denoise_image_par)

This function implements a basic denoising algorithm that averages the color values of a pixel's neighbors to reduce noise in parallel

2.0) Gaussian Denoise (denoise_image_imp)

This function implements an improved denoising algorithm using a Gaussian blur technique. 
It generates a Gaussian kernel matrix and applies it to each pixel and its neighbors in the input image to reduce noise.

## Conclusion from project

These denoising functions provide a simple and efficient way to reduce noise in images. 
The basic denoising function (denoise_image) offers a simple averaging approach and is fast enough for smaller batches.
The basic denoising function in parallel (denoise_image_par) offers the same as above but we do not see the full benefit of parallel computing as the workload is not large enough.
while the improved denoising function (denoise_image_imp) utilizes Gaussian blur for better noise reduction and is blaZingly fast for the quality if offers.
