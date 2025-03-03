// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();
use image;
//use std::io;
use std::error::Error;
use std::path::{Path, PathBuf};
use rayon::prelude::*;
use rfd::FileDialog;
use slint::SharedString;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    ui.on_request_file(move || {
        if let Some(path) = FileDialog::new().pick_file() {
            let path = path.to_str().unwrap();
            //ui.set_file_path(&path);
            match split_image(&path) {
                Ok(()) => {
                    if let Some(ui) = ui_handle.upgrade() {
                        let path = format!("Images saved to: {}", Path::new(&path).parent().unwrap().to_str().unwrap());
                        ui.set_file_path(SharedString::from(path));
                    }
                }
                Err(e) => {
                    if let Some(ui) = ui_handle.upgrade() {
                        let error_string = format!("Error: {}", e);
                        ui.set_file_path(SharedString::from(error_string));
                    }
                }
            }
        }
    });

    ui.run()?;

    Ok(())
}

fn split_image(input: &str) -> Result<(), Box<dyn Error>> {
    let image_path = format!("{}", input.trim());
    let parent_path = Path::new(&image_path).parent().unwrap();
    let image_name = Path::new(&image_path).file_stem().unwrap().to_str().unwrap();

    let image = image::open(&image_path).map_err(|e| {
        println!("Aborting function: {}", e);
        e
    })?;

    let (width, height) = (image.width(), image.height());
    let (half_width, half_height) = (width / 2, height / 2);

    let filenames = [ "top_left.png", "top_right.png", "bottom_left.png", "bottom_right.png" ];

    let crops = vec![
        (0, 0, filenames[0]),
        (half_width, 0, filenames[1]),
        (0, half_height, filenames[2]),
        (half_width, half_height, filenames[3]),
    ];

    crops.par_iter().for_each(|(x, y, filename)| {
        let mut image_clone = image.clone();
        let crop = image_clone.crop(*x, *y, half_width, half_height);
        let filename = format!("{}-{}", image_name, filename);
        let save_path = PathBuf::from(parent_path).join(filename);
        println!("Saving: {}", save_path.to_str().unwrap());
        crop.save(save_path).unwrap();
    });

    Ok(())
}

