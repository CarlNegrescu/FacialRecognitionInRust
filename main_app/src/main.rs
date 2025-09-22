//! main.rs
//! 
//! Main application entry point 
//! 
//! 09/21/2025

use camera_capture::Camera;
use minifb::{Key, Window, WindowOptions};

const WINDOW_WIDTH: usize  = 640;
const WINDOW_LENGTH: usize = 360;

fn main() {
    
    let mut camera  = match Camera::new() {
        Ok(cam) => cam,
        Err(e) => {
            eprintln!("ERROR: Failed to initialize camera: {}", e);
            return;
        }
    };
    // if open_stream fails cannot continue 
    if let Err(e) = camera.open_stream() {
        eprintln!("ERROR: Failed to open camera stream: {}", e);
        return;
    }

    let mut window = match Window::new(
        "Face Frame RS - Press ESC to exit",
        WINDOW_WIDTH,
        WINDOW_LENGTH,
        WindowOptions::default(),
    ) {
        Ok(win) => win,
        Err(e) => {
            eprintln!("ERROR: Failed to create window: {}", e);
            return;
        }
    };

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60 FPS
    println!("Camera feed is live. Press ESC in the window to exit.");

    while window.is_open() && !window.is_key_down(Key::Escape) {
    // Capture a frame from the camera.
        match camera.capture_frame() {
            Ok(frame) => {
                println!("Original frame dimensions: {}x{}", frame.width(), frame.height());
                let rgb_frame = frame.to_rgb8();

                let buffer: Vec<u32> = rgb_frame
                    .pixels()
                    .map(|p| {
                        let r = p[0] as u32;
                        let g = p[1] as u32;
                        let b = p[2] as u32;
                        (r << 16) | (g << 8) | b
                    })
                    .collect();
                
                let expected_len = WINDOW_WIDTH * WINDOW_LENGTH;
                println!("Buffer len: {}, Expected window len: {}", buffer.len(), expected_len);
                if buffer.len() != expected_len {
                    println!("MISMATCH DETECTED: Buffer size does not match window size!");
                }
                // Update the window with the new frame data.
                if let Err(e) = window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_LENGTH) {
                    eprintln!("ERROR: Failed to update window buffer: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("ERROR: Could not capture frame: {}", e);
                break;
            }
        }
    }
}
