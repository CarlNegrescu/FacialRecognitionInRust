# FacialRecognitionInRust#

A real-time facial recognition application built with Rust to learn the language and explore computer vision.

[![Rust](https://img.shields.io/badge/rust-1.79-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## About The Project

FaceFrame RS is a project created to learn the Rust programming language by building a practical, high-performance application. The goal is to create a complete facial recognition system, from capturing a video feed to identifying known individuals.

The project is structured as a Cargo workspace to promote modular and clean code.

### Current Status

✅ **Working!** The application can successfully:
* Connect to a system webcam.
* Capture live video frames.
* Display the video feed in a desktop window at ~60 FPS.

The core facial detection and recognition logic has not yet been implemented.

---

## 🚀 Getting Started

To get a local copy up and running, follow these simple steps.

### Prerequisites

* **Rust Toolchain**: Make sure you have Rust installed. You can get it from [rustup.rs](https://rustup.rs/).
* **A Webcam**: The application requires a connected camera.

### Installation & Usage

1.  **Clone the repository:**

2.  **Navigate to the project directory:**
    ```sh
    cd your_repository_name
    ```
3.  **Run the main application:**
    Since this is a Cargo workspace, you need to specify which package to run.
    ```sh
    cargo run --package main_app
    ```
4.  A window showing your camera feed should appear. Press the **ESC** key to close it.

---

## 🏗️ Project Structure

This project is a Cargo workspace containing multiple crates:

* `camera_capture/`: A library crate responsible for abstracting all camera interactions. It provides a simple API for the main application to use.
* `facial_detection_library/`: (Placeholder) This crate will contain the logic for detecting faces in an image.
* `main_app/`: The main binary crate that ties everything together. It handles the windowing, event loop, and displays the final output.

---

## 🛣️ Roadmap

* [ ] Integrate a facial **detection** model to find faces in the video stream.
* [ ] Implement a facial **embedding** model to create unique identifiers for detected faces.
* [ ] Add a simple database or file system to store and manage known faces.
* [ ] Draw bounding boxes and names over faces in the live video feed.

---

## Built With

* [Rust](https://www.rust-lang.org/)
* [nokhwa](https://crates.io/crates/nokhwa) - For cross-platform camera capture.
* [minifb](https://crates.io/crates/minifb) - For creating a simple, cross-platform window.
* [image](https://crates.io/crates/image) - For image processing and resizing.