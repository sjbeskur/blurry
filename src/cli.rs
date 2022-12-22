use clap::{Parser, Subcommand};

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
    }
}