use std::fs::File;
use std::path::Path;
use std::env;
use std::process;
use std::io::Read;

struct Source {
    source_code: Vec<u8>,
    source_len: usize,
    char_pointer: usize,
}

impl Source {
    fn read(&mut self) -> Option<&u8> {
        let c = self.source_code.get(self.char_pointer);
        self.char_pointer += 1;
        return c
    }

    fn peek(&mut self) -> Option<&u8> {
        return self.source_code.get(self.char_pointer);
    }
}

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

    let mut source_text = String::new();
    match file.read_to_string(&mut source_text) {
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(3)
        },
        Ok(_) => ()
    }

    let ascii_source: Vec<u8> = source_text.into_bytes();

    let mut source = Source {
        source_len: ascii_source.len(),
        source_code: ascii_source,
        char_pointer: 0
    };


    while source.peek() != None {
        match source.read() {
            Some(b'+') => {
                println!("Plus!")
            },
            Some(_) => (),
            None => println!("None.")
        }
    }
}
