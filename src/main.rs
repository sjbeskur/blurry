fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f = args[1].clone();
    let k = args[2].clone().parse::<i32>().unwrap();
    if let Err(e) = blurry::run(f,k){
        eprintln!("{}", e);
    }
}
