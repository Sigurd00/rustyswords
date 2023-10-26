use opencv::{imgcodecs, highgui};
use rustyswords::capture::capture_screen;


fn main() {
    //let _image = imgcodecs::imread("Monner.png", -1).unwrap();
    
    let screenshot = capture_screen().unwrap();
    highgui::named_window("hello opencv!", 0).unwrap();
    highgui::imshow("hello opencv!", &screenshot).unwrap();
    highgui::wait_key(1000000).unwrap();
}