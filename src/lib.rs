#![allow(dead_code)]
use opencv as cv;
use cv::{
    prelude::*,
    core,
    imgcodecs,
    imgproc,
    types::VectorOfu8,
};

pub type BoxedError = Box<dyn std::error::Error>;
pub type AppResult<T> = Result<T, BoxedError>;

pub mod cli;
use cli::*;

pub fn blur_gaussian(config: Config, ksize: i32, sigma_x: f64, sigma_y: f64) -> AppResult<()>{    

    if ksize%2 == 0{
        panic!("invalid ksize {}", ksize);
    }
    let m = ksize;
    let n = ksize;

    let src = imgcodecs::imread(&config.filename, imgcodecs::IMREAD_COLOR)?;

    let mut dst = Mat::default();
    let size = core::Size::new(m,n);

    imgproc::gaussian_blur(&src, &mut dst, size,  sigma_x, sigma_y, core::BORDER_REFLECT_101)?;

    let name = format!("{} (k:{}, s:{})", config.filename, ksize, sigma_x);
    show(name, &dst)?;
    Ok(())
}

pub fn blur_avg(config: Config, ksize: i32) -> AppResult<()>{    
    if ksize%2 == 0{
        panic!("invalid ksize {}", ksize);
    }
    let m = ksize;
    let n = ksize;

    let src = imgcodecs::imread(&config.filename, imgcodecs::IMREAD_COLOR)?;

    let mut dst = Mat::default();
    let size = core::Size::new(m,n);
    let anchor = core::Point::new(-1,-1);
    imgproc::blur(&src, &mut dst, size,  anchor, core::BORDER_REFLECT_101)?;

    show(config.filename, &dst)?;
    
    Ok(())
}

fn show(name: String, img: &Mat ) -> AppResult<()>{
    cv::highgui::named_window(&name, cv::highgui::WINDOW_KEEPRATIO)?;
    cv::highgui::imshow(&name, img)?;
    let _key = cv::highgui::wait_key(0)?;
    cv::highgui::destroy_all_windows()?;
    Ok(())
}