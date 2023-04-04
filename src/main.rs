extern crate opencv;
use screenshots::{Image, Screen};
use opencv::{
    core, highgui, imgcodecs, imgproc,
    prelude::*,
    Result,
};

fn match_template(template: &Mat, target: &Mat) -> Result<Mat> {
    let mut result = Mat::default();
    imgproc::match_template(
        target,
        template,
        &mut result,
        imgproc::TM_CCOEFF_NORMED,
        &Mat::default(),
    )?;
    Ok(result)
}

fn run() -> Result<()> {
    // Load the template image
    let template_path = "template.png";
    let template = imgcodecs::imread(template_path, imgcodecs::IMREAD_COLOR)?;

    //TODO Sigurd: Bestem hvilken skærm der kører spillet og tag kun skræmbilled af den?
    let screens = Screen::all().unwrap();

    loop {
        // Capture a screenshot of the desktop

        let mut images: Vec<Mat> = vec![];
        for screen in &screens {
            println!("capturer {screen:?}");
            let image = screen.capture().unwrap();
            //fs::write(format!("target/{}.png", screen.display_info.id), buffer).unwrap();
            
            println!("image {:?}, {:?}, {:?}", image.buffer().len(), image.height(), image.width());
            images.push(screenshot_to_mat(image).unwrap());
        }

        // Match the template to the screenshot
        let results: Vec<Mat> =  images.iter().map(|i| match_template(&template, i).unwrap()).collect();

        // Find the maximum value and its location in the result matrix
        let mut min_val = 0.0;
        let mut max_val = 0.0;
        let mut min_loc = core::Point::default();
        let mut max_loc = core::Point::default();
        core::min_max_loc(&results.first().unwrap(), Some(&mut min_val), Some(&mut max_val), Some(&mut min_loc), Some(&mut max_loc), &core::no_array())?;

        // Draw a rectangle around the matched region in the screenshot
        let (w, h) = (template.cols(), template.rows());
        let rect = core::Rect::new(max_loc.x, max_loc.y, w, h);
        imgproc::rectangle(
            &mut images.first().unwrap().to_owned(),
            rect,
            core::Scalar::all(255.0),
            2,
            imgproc::LINE_8,
            0,
        )?;

        // Display the screenshot with the matched region highlighted
        highgui::imshow("Did he find it?!?", &images.first().unwrap())?;
        highgui::wait_key(1000)?;

        // Exit the loop if the user presses the Esc key
        if highgui::wait_key(1)? == 27 {
            break;
        }
    }

    Ok(())
}

fn screenshot_to_mat(image: Image) -> opencv::Result<Mat> {
    Mat::from_slice_rows_cols(image.buffer(), image.height() as usize, image.width() as usize)
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
    }
}
