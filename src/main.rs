extern crate opencv;
use opencv::imgcodecs::IMREAD_COLOR;
use opencv::imgcodecs::imread;

use opencv::{
    core,
    highgui,
    prelude::*,
    Result,
};

fn run() -> Result<()> {
    // Load an image
    let filename = "Sad cat.png";
    let img = imread(filename, IMREAD_COLOR)?;

    // Display the image
    highgui::imshow("image", &img)?;
    highgui::wait_key(0)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
    }
}