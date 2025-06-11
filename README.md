# Image Denoising in Rust

A high-performance image denoising implementation showcasing various algorithms and parallel processing techniques in Rust. This project demonstrates computer vision concepts, parallel computing, and modern Rust programming practices.

## Table of Contents
- [Features](#features)
- [Implementation Details](#implementation-details)
  - [Original Implementations](#original-implementations)
  - [Improved Implementations](#improved-implementations)
- [Usage](#usage)
- [Learning Outcomes](#learning-outcomes)
- [Running Benchmarks](#running-benchmarks)
- [License](#license)

## Features

- **Basic Denoising**: Simple averaging-based noise reduction
- **Gaussian Denoising**: Advanced noise reduction using Gaussian kernels
- **Bilateral Filtering**: Edge-preserving noise reduction
- **LAB Color Space Processing**: Improved color preservation
- **Parallel Processing**: Multi-threaded implementations using Rayon

## Implementation Details

### Original Implementations
Located in `src/original/`:

1. **Basic Denoising** (`img_denoise.rs`)
   - Simple averaging of neighboring pixels
   - Configurable kernel radius
   - Sequential processing

2. **Basic Parallel Denoising** (`img_denoise_par.rs`)
   - Parallel version of basic denoising
   - Uses Rayon for thread management
   - Configurable thread count

3. **Gaussian Denoising** (`img_denoise_gaussian.rs`)
   - Gaussian kernel-based denoising
   - Configurable sigma and kernel radius
   - Sequential processing

4. **Parallel Gaussian Denoising** (`image_denoise_gaussian_par.rs`)
   - Parallel version of Gaussian denoising
   - Uses Rayon for thread management
   - Configurable thread count

### Improved Implementations
Located in `src/improved/`:

1. **Improved Basic Denoising** (`img_denoise_improved.rs`)
   - Enhanced averaging with weighted neighbors
   - Better edge preservation
   - Parallel processing with work stealing

2. **Gaussian Denoising with LAB** (`img_denoise_gaussian_improved.rs`)
   - Gaussian denoising in LAB color space
   - Better color preservation
   - Adaptive kernel weights
   - Parallel processing with optimized thread distribution
   - Edge-aware processing

3. **Bilateral Filtering** (`img_denoise_bilateral.rs`)
   - Edge-preserving denoising
   - Separate spatial and range weights
   - LAB color space processing
   - Parallel implementation with work stealing
   - Adaptive parameter tuning

## Usage

```rust
// Example: Using the improved Gaussian denoising
let denoised = denoise_image_gaussian_improved(
    &image,
    kernel_radius,  // Size of the neighborhood (e.g., 2)
    sigma,         // Gaussian sigma for weight calculation (e.g., 1.5)
    num_threads    // Number of threads for parallel processing (e.g., 4)
);

// Example: Using bilateral filtering
let denoised = denoise_image_bilateral(
    &image,
    kernel_radius,  // Size of the neighborhood
    sigma,         // Sigma for range weight calculation
    num_threads    // Number of threads
);
```

## Learning Outcomes

This project demonstrates:
- Computer vision algorithms and image processing
- Parallel computing in Rust using Rayon
- Color space transformations (RGB to LAB)
- Memory management and optimization in Rust
- Benchmarking and profiling techniques

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench denoising_benchmarks
```

The benchmarks compare:
- All denoising algorithms (basic, Gaussian, bilateral)
- Sequential vs parallel implementations
- Different kernel sizes (2, 4, 6)
- Various parameter configurations

Benchmark Configuration:
- 20 samples per test for statistical confidence
- 15 seconds measurement time
- 3 seconds warm-up time
- 99% confidence level
- 1% significance level
- Flat sampling mode for consistent results

## License

MIT License - See LICENSE file for details
