use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(author="", version="", about="", long_about = None)]
pub struct Config{
    pub filename: String,

    
    #[command(subcommand)]
    pub command: Commands,    
}

#[derive(Debug, Subcommand)]
pub enum Commands{
    #[command(author, version, about, long_about = "Blurs using normalized box filter")]
    Norm {
        #[arg(short, long, help="box filter kernel size (should be odd number)")]
        ksize: i32,
    },

    #[command(author, version, about, long_about = "Blurs an image using a Gaussian filter")]
    Gaus{
        #[arg(short='k', long)]
        ksize: i32,
   
        #[arg(short='s', long, default_value_t=0.0)]
        sigma: f64,

        #[arg(short='b', value_enum, default_value_t=BorderStrategy::Reflect)]
        boarder: BorderStrategy,
    },

    #[command(author, version, about, long_about = "Blurs using the median filter")]
    Median {
        #[arg(short, long, help="aperture linear size; it must be odd and greater than 1, for example: 3, 5, 7 ")]
        ksize: i32,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug)]
#[repr(i32)]
pub enum BorderStrategy{
    Reflect = opencv::core::BORDER_REFLECT101,
    Replicate = opencv::core::BORDER_REPLICATE,
    Wrap = opencv::core::BORDER_WRAP,
}
