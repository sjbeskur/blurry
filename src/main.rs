use clap::{Parser, Subcommand};

fn main() {

    let cfg = blurry::cli::Config::parse();
    dbg!(&cfg);

    match cfg.command {
        Some(blurry::cli::Commands::Avg{ ksize }) => {
            println!("{}", ksize);
            blurry::blur_avg(cfg, ksize);
        },
        Some(blurry::cli::Commands::Gaus{ksize, sigma}) => {
            println!("Gaussian");
            blurry::blur_gaussian(cfg, ksize, sigma, sigma);
        }
        None=>{}
        
    }
/* 
    if let Err(e) = blurry::run(cfg){
        eprintln!("{}", e);
    }
*/
}
