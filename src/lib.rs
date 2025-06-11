pub mod original {
    pub mod img_denoise;
    pub mod img_denoise_gaussian;
    pub mod img_denoise_par;
    pub mod image_denoise_gaussian_par;
}

pub mod improved {
    pub mod img_denoise_improved;
    pub mod img_denoise_bilateral;
    pub mod img_denoise_gaussian_improved;
}

// Re-export the denoising functions
pub use original::img_denoise::denoise_image;
pub use original::img_denoise_par::denoise_image_par;
pub use original::img_denoise_gaussian::denoise_image_gaussian;
pub use original::image_denoise_gaussian_par::denoise_image_gaussian_par;
pub use improved::img_denoise_gaussian_improved::denoise_image_gaussian_improved;
pub use improved::img_denoise_bilateral::denoise_image_bilateral; 