use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, SamplingMode};
use std::time::Duration;
use std::fs;
use rayon::prelude::*;
use image::{ImageBuffer, Rgb};

use image_denoising::{
    denoise_image,
    denoise_image_par,
    denoise_image_gaussian,
    denoise_image_gaussian_par,
    denoise_image_gaussian_improved,
    denoise_image_bilateral,
};

// Get all test images from the test_file directory
fn get_test_images() -> Vec<String> {
    let test_dir = "test_file";
    fs::read_dir(test_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| {
                matches!(ext.to_str().unwrap().to_lowercase().as_str(), "jpg" | "jpeg" | "png")
            }) {
                Some(path.to_str().unwrap().to_string())
            } else {
                None
            }
        })
        .collect()
}

// Pre-process images to avoid I/O during benchmarking
fn preprocess_images(test_images: &[String]) -> Vec<(String, ImageBuffer<Rgb<u8>, Vec<u8>>)> {
    test_images.par_iter()
        .map(|path| {
            let image = image::open(path).unwrap();
            let rgb_image = image.to_rgb8();
            (path.clone(), rgb_image)
        })
        .collect()
}

fn bench_denoising(c: &mut Criterion) {
    let test_images = get_test_images();
    if test_images.is_empty() {
        panic!("No test images found in test_file directory!");
    }

    // Pre-process all images
    let processed_images = preprocess_images(&test_images);

    let mut group = c.benchmark_group("Denoising Algorithms");
    
    // Configure the benchmark group for efficient sampling
    group.measurement_time(Duration::from_secs(15))
         .warm_up_time(Duration::from_secs(3))
         .sample_size(20)                           // Keep 20 samples for good statistical confidence
         .sampling_mode(SamplingMode::Flat)        // Use flat sampling for more consistent results
         .confidence_level(0.99)                    // High confidence
         .significance_level(0.01);                 // Strict significance level

    // Create output directory if it doesn't exist
    std::fs::create_dir_all("res_dump").unwrap();

    // For each test image, benchmark all algorithms
    for (i, (input_path, image)) in processed_images.iter().enumerate() {
        let output_path = format!("res_dump/benchmark_output_{}.jpeg", i);
        let output_path = output_path.as_str();

        // Basic denoising
        group.bench_with_input(
            BenchmarkId::new("basic", format!("image_{}", i)),
            &(input_path, image),
            |b, (path, img)| {
                b.iter(|| {
                    let temp_path = format!("res_dump/temp_input_{}.jpeg", i);
                    img.save(&temp_path).unwrap();
                    denoise_image(black_box(path), black_box(output_path));
                    std::fs::remove_file(temp_path).unwrap();
                })
            }
        );

        // Parallel basic denoising
        group.bench_with_input(
            BenchmarkId::new("basic_parallel", format!("image_{}", i)),
            &(input_path, image),
            |b, (path, img)| {
                b.iter(|| {
                    let temp_path = format!("res_dump/temp_input_{}.jpeg", i);
                    img.save(&temp_path).unwrap();
                    denoise_image_par(black_box(path), black_box(output_path));
                    std::fs::remove_file(temp_path).unwrap();
                })
            }
        );

        // Gaussian denoising
        group.bench_with_input(
            BenchmarkId::new("gaussian", format!("image_{}", i)),
            &(input_path, image),
            |b, (path, img)| {
                b.iter(|| {
                    let temp_path = format!("res_dump/temp_input_{}.jpeg", i);
                    img.save(&temp_path).unwrap();
                    denoise_image_gaussian(
                        black_box(path),
                        black_box(output_path),
                        black_box(2.0),
                        black_box(1.5)
                    );
                    std::fs::remove_file(temp_path).unwrap();
                })
            }
        );

        // Parallel Gaussian denoising
        group.bench_with_input(
            BenchmarkId::new("gaussian_parallel", format!("image_{}", i)),
            &(input_path, image),
            |b, (path, img)| {
                b.iter(|| {
                    let temp_path = format!("res_dump/temp_input_{}.jpeg", i);
                    img.save(&temp_path).unwrap();
                    denoise_image_gaussian_par(
                        black_box(path),
                        black_box(output_path),
                        black_box(2.0),
                        black_box(1.5)
                    );
                    std::fs::remove_file(temp_path).unwrap();
                })
            }
        );

        // Improved Gaussian denoising
        group.bench_with_input(
            BenchmarkId::new("gaussian_improved", format!("image_{}", i)),
            &(input_path, image),
            |b, (path, img)| {
                b.iter(|| {
                    let temp_path = format!("res_dump/temp_input_{}.jpeg", i);
                    img.save(&temp_path).unwrap();
                    denoise_image_gaussian_improved(
                        black_box(path),
                        black_box(output_path),
                        black_box(2),
                        black_box(1.5)
                    );
                    std::fs::remove_file(temp_path).unwrap();
                })
            }
        );

        // Bilateral filtering
        group.bench_with_input(
            BenchmarkId::new("bilateral", format!("image_{}", i)),
            &(input_path, image),
            |b, (path, img)| {
                b.iter(|| {
                    let temp_path = format!("res_dump/temp_input_{}.jpeg", i);
                    img.save(&temp_path).unwrap();
                    denoise_image_bilateral(
                        black_box(path),
                        black_box(output_path),
                        black_box(4),
                        black_box(1.5),
                        black_box(0.1)
                    );
                    std::fs::remove_file(temp_path).unwrap();
                })
            }
        );
    }

    group.finish();
}

// Benchmark different kernel sizes for each test image
fn bench_image_sizes(c: &mut Criterion) {
    let test_images = get_test_images();
    if test_images.is_empty() {
        panic!("No test images found in test_file directory!");
    }

    // Pre-process all images
    let processed_images = preprocess_images(&test_images);

    let mut group = c.benchmark_group("Image Sizes");
    
    // Configure the benchmark group for efficient sampling
    group.measurement_time(Duration::from_secs(15))
         .warm_up_time(Duration::from_secs(3))
         .sample_size(20)                           // Keep 20 samples for good statistical confidence
         .sampling_mode(SamplingMode::Flat)        // Use flat sampling for more consistent results
         .confidence_level(0.99)                    // High confidence
         .significance_level(0.01);                 // Strict significance level

    // Test with different kernel sizes
    let kernel_sizes = [2, 4, 6];
    
    // Test on all images
    for (i, (input_path, image)) in processed_images.iter().enumerate() {
        for &kernel_size in kernel_sizes.iter() {
            let output_path = format!("res_dump/benchmark_output_{}_kernel_{}.jpeg", i, kernel_size);
            let output_path = output_path.as_str();
            
            group.bench_with_input(
                BenchmarkId::new(
                    format!("gaussian_improved_image_{}", i),
                    format!("kernel_size_{}", kernel_size)
                ),
                &(input_path, image),
                |b, (path, img)| {
                    let ksize = kernel_size;  // Capture kernel_size in closure
                    b.iter(|| {
                        let temp_path = format!("res_dump/temp_input_{}_kernel_{}.jpeg", i, ksize);
                        img.save(&temp_path).unwrap();
                        denoise_image_gaussian_improved(
                            black_box(path),
                            black_box(output_path),
                            black_box(ksize),
                            black_box(1.5)
                        );
                        std::fs::remove_file(temp_path).unwrap();
                    })
                },
            );
        }
    }

    group.finish();
}

criterion_group!(benches, bench_denoising, bench_image_sizes);
criterion_main!(benches); 