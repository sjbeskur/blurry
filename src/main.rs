use clap::{Parser, Subcommand};

fn main() {

    let cfg = blurry::cli::Config::parse();
    dbg!(&cfg);

    match cfg.command {
        
        blurry::cli::Commands::Norm{ ksize } => {
            println!("{}", ksize);
            blurry::blur_avg(cfg, ksize);
        },

        blurry::cli::Commands::Gaus{ksize, sigma} => {
            println!("Gaussian");
            blurry::blur_gaussian(cfg, ksize, sigma, sigma);
        }
        
    }

/* 
    if let Err(e) = blurry::run(cfg){
        eprintln!("{}", e);
    }
*/
}
