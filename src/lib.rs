#![allow(dead_code)]
use opencv as cv;
use cv::{
    prelude::*,
    core,
    imgcodecs,
    imgproc,
    types::VectorOfu8,
};

type BoxedError = Box<dyn std::error::Error>;
type AppResult<T> = Result<T, BoxedError>;

pub fn run(filename: String, ksize: i32) -> AppResult<()>{    
    if ksize%2 == 0{
        panic!("invalid ksize {}", ksize);
    }
    println!("{}", "here");
    let m = ksize;
    let n = ksize;

    println!("{}", filename);
    let src = imgcodecs::imread(&filename, imgcodecs::IMREAD_COLOR)?;

    let mut dst = Mat::default();
    let size = core::Size::new(m,n);
    let anchor = core::Point::new(-1,-1);
    imgproc::blur(&src, &mut dst, size,  anchor, core::BORDER_REFLECT_101)?;

    show("sam".to_string(), &dst)?;
    
    Ok(())
}

fn show(name: String, img: &Mat ) -> AppResult<()>{
    cv::highgui::named_window(&name, cv::highgui::WINDOW_KEEPRATIO)?;
    cv::highgui::imshow(&name, img)?;
    let _key = cv::highgui::wait_key(0)?;
    cv::highgui::destroy_all_windows()?;
    Ok(())
}