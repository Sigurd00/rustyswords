use std::{error::Error, fmt};

use opencv::{prelude::Mat, core::{Mat_AUTO_STEP, CV_8UC3}};
use x11::xlib::{XOpenDisplay, XDefaultScreen, XRootWindow, XDisplayHeight, XDisplayWidth, XImage, XGetImage, ZPixmap, XDestroyImage, XCloseDisplay};

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn capture_screen() -> Result<Mat, CaptureError> {
    let display = unsafe { XOpenDisplay(std::ptr::null()) };
    if display.is_null() {
        return Err(CaptureError::new("Failed to open X display"));
    }

    let screen = unsafe { XDefaultScreen(display) };
    let root = unsafe { XRootWindow(display, screen) };
    let width = unsafe { XDisplayWidth(display, screen) };
    let height = unsafe { XDisplayHeight(display, screen) };

    let mut image: XImage = XImage {
        width,
        height,
        xoffset: 0,
        format: ZPixmap,
        data: std::ptr::null_mut(),
        byte_order: 0,
        bitmap_unit: 0,
        bitmap_bit_order: 0,
        bitmap_pad: 0,
        depth: 24,
        bytes_per_line: 0,
        bits_per_pixel: 0,
        red_mask: 0,
        green_mask: 0,
        blue_mask: 0,
        obdata: std::ptr::null_mut(),
        funcs: x11::xlib::ImageFns {
            create_image: None,
            destroy_image: None,
            get_pixel: None,
            put_pixel: None,
            sub_image: None,
            add_pixel: None,
        },
    };

    let image = unsafe {
        XGetImage(
            display,
            root,
            0,
            0,
            width as u32,
            height as u32,
            !0,
            ZPixmap,
        )
    };
    println!("{}, {}, {}", width, height, unsafe {(*image).bytes_per_line});    
    if image.is_null() {
        return Err(CaptureError::new("Failed to get XImage"));
    }

    // Create an OpenCV Mat from the image data
    let data = unsafe {
        std::slice::from_raw_parts(
            (*image).data as *const u8,
            ((*image).bytes_per_line * height as i32) as usize,
        )
    };

    let mat = unsafe { Mat::new_rows_cols_with_data(
        height,
        width,
        CV_8UC3,
        data.as_ptr() as *mut std::ffi::c_void,
        (width * 3) as usize,
    ) };

    // Clean up resources
    unsafe {
        XDestroyImage(image);
        XCloseDisplay(display);
    }

    if let Ok(mat) = mat {
        Ok(mat)
    } else {
        Err(CaptureError::new("Failed to create OpenCV Mat"))
    }
}

#[derive(Debug)]
pub struct CaptureError {
    message: String,
}

impl CaptureError {
    fn new(message: &str) -> Self {
        CaptureError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for CaptureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Capture Error: {}", self.message)
    }
}

impl Error for CaptureError {}
