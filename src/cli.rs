use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author="", version="", about="", long_about = None)]
pub struct Config{
    pub filename: String,

    
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands{
    Avg {
        #[arg(short, long)]
        ksize: i32,
    },

    Gaus{
        #[arg(short='k', long)]
        ksize: i32,
   
        #[arg(short='s', long)]
        sigma: f64,
    }
}