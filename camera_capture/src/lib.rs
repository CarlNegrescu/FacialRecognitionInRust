//! # Camera Capture Library
//!
//! A high-level, cross-platform library for capturing frames from a camera.
//! This crate abstracts the backend-specific details of `nokhwa`.
//! 
//! 09/21/2025

use image::DynamicImage;
use nokhwa::{
    query,
    pixel_format:: RgbFormat,
    utils::{ApiBackend, RequestedFormat, RequestedFormatType, 
            CameraFormat, FrameFormat, Resolution},
    Camera as NokhwaCamera,
};
use thiserror::Error;

const FPS: u32 = 30;
/// Custom error types for possible camera failures
#[derive(Error, Debug)]
pub enum CameraError {
    #[error("No suitable camera found.")]
    NoCameraFound,
    
    #[error("Failed to initialize camera: {0}")]
    InitializationFailed(#[from] nokhwa::NokhwaError),
    
    #[error("Failed to decode frame into an image.")]
    FrameDecodeError,

    #[error("Camera stream not started")]
    StreamNotOpen
}

/// Camera connection 
pub struct Camera {
    nokhwa_cam: NokhwaCamera,
    is_open: bool,
}

impl Camera {

    /// Creates a new connection to a camera
    /// It will attempt to find the best available camera
    /// and configure it with a default frame rate
    /// 
    /// # Returns
    /// A `Result` containing the `Camera` instance or a `CameraError`
pub fn new() -> Result<Self, CameraError> {
    let cameras = query(ApiBackend::Auto)?;
    if cameras.is_empty() {
        return Err(CameraError::NoCameraFound);
    }
    let camera_info = cameras.get(0).ok_or(CameraError::NoCameraFound)?;
    let camera_index = camera_info.index().clone();

    let format = CameraFormat::new(
        Resolution::new(640, 360),
        FrameFormat::NV12, 
        FPS,
    );
    
    let nokhwa_cam = NokhwaCamera::new(
        camera_index,
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::Exact(format)),
    )?;

    Ok(Self { nokhwa_cam, is_open: false })
}

    /// Opens the camera stream.
    /// Must be called before capturing frames
    pub fn open_stream(&mut self) -> Result<(), CameraError>{

        self.nokhwa_cam.open_stream()?;
        self.is_open = true;

        return Ok(());
    }

    /// Captures a single frame from the camera stream
    /// 
    /// # Returns
    /// 
    /// A `Result` containing the captured frame as a `DynamicImage`
    ///  or a `CameraError`
    pub fn capture_frame(&mut self) -> Result<DynamicImage, CameraError> {
        if self.is_open {
            let frame = self.nokhwa_cam.frame()?;
            let decoded = frame.decode_image::<RgbFormat>()
                .map_err(|_| CameraError::FrameDecodeError)?;
            
            return Ok(DynamicImage::ImageRgb8(decoded));
        }
        else {
            return Err(CameraError::StreamNotOpen);
        }
       
        
        
    }

}