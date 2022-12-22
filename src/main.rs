use clap::{Parser};

fn main() -> blurry::AppResult<()>{

    let cfg = blurry::cli::Config::parse();
    dbg!(&cfg);

    match cfg.command {
        
        blurry::cli::Commands::Norm{ ksize } => {
            println!("{}", ksize);
            blurry::blur_avg(cfg, ksize)
        },

        blurry::cli::Commands::Gaus{ksize, sigma} => {
            println!("Gaussian");
            blurry::blur_gaussian(cfg, ksize, sigma, sigma)
        }
        
    }

/* 
    if let Err(e) = blurry::run(cfg){
        eprintln!("{}", e);
    }
*/
}



#[test]
fn verify_cli() {
    use clap::CommandFactory;
    blurry::cli::Config::command().debug_assert()
}