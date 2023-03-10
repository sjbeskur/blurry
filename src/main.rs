use clap::{Parser};

fn main() {
    let cfg = blurry::cli::Config::parse();
    if let Err(e) = run_subcommand(cfg){
        eprintln!("{}", e);
        std::process::exit(1)
    };

}

fn run_subcommand(cfg: blurry::cli::Config) -> blurry::AppResult<()>{
    match cfg.command {
        
        blurry::cli::Commands::Norm{ ksize } => {
            println!("{}", ksize);
            blurry::blur_norm(cfg, ksize)
        },

        blurry::cli::Commands::Gaus{ksize, sigma, boarder} => {
            blurry::blur_gaussian(cfg, ksize, sigma, sigma, boarder)
        },
        
        blurry::cli::Commands::Median{ksize} => {
            blurry::blur_median(cfg, ksize)
        },
    }

}


#[test]
fn verify_cli() {
    use clap::CommandFactory;
    blurry::cli::Config::command().debug_assert()
}