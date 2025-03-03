
# Instagram Image Splitter (Rust)

A simple Rust program that splits an image into 4 quadrants, making it suitable for posting on Instagram (which has a 1080px image size limit). This way, you can avoid losing image detail when posting large images by splitting them into smaller, square pieces.

## Features

- **Splits an image into 4 quadrants**.
- Each quadrant is designed for Instagram’s square post format.
- **Saves each quadrant as a separate file**.
- **Cross-platform**: Can be compiled on various operating systems.

## Dependencies

To build the project, you'll need the following dependencies:

- **[Rust](https://www.rust-lang.org/)**: The programming language used.
- **[Image](https://docs.rs/image/latest/image/)**: Used for image manipulation (loading, cropping, saving).
- **[Rayon](https://docs.rs/rayon/latest/rayon/)**: A library for parallel iteration to speed up processing.
- **[RFD](https://docs.rs/rfd/latest/rfd/)**: A library for file dialogs, which allows the user to select input images easily.
- **[Slint](https://slint.dev/)**: A UI framework to create a simple interface for interacting with the program.

## How to Build

1. **Clone the repository:**

   ```bash
   git clone https://github.com/pezfisk/RustInstagramSplitter
   cd RustInstagramSplitter
   ```

2. **Install the dependencies** (make sure you have [Rust](https://www.rust-lang.org/learn/get-started) installed).

3. **Build the project** using Cargo:

   ```bash
   cargo build --release
   ```

4. The output binary will be located in the `target/release` directory.

## How to Use

1. Run the program:

   ```bash
   cargo run
   ```

2. When prompted, select an image file from your file explorer.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
