use std::fs::File;
use std::path::Path;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No arguments provided.");
        process::exit(1);
    }

    let path = Path::new(&args[1]);
    println!("{}", path.display());

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            process::exit(2)
        }
    };
}
