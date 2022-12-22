use clap::{Parser, Subcommand};

fn main() {

    let cfg = blurry::cli::Config::parse();
    dbg!(&cfg);

    match cfg.command {
        Some(blurry::cli::Commands::Avg{ ksize }) => {
            println!("{}", ksize);
            blurry::run(cfg, ksize);
        },
        Some(blurry::cli::Commands::Gaus) => {
            println!("Gaussian");
        }
        None=>{}
        
    }
/* 
    if let Err(e) = blurry::run(cfg){
        eprintln!("{}", e);
    }
*/
}
