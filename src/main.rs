use std::fs::File;
use std::path::Path;
use std::env;
use std::process;
use std::io::Read;
use std::io;

struct Source {
    source_code: Vec<u8>,
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

struct Tape {
    source: Source,
    tape: Box<[u8; 30000]>, 
    tape_ptr: usize,
}

impl Tape {
    fn shift_right(&mut self) {
        self.tape_ptr += 1;
    }

    fn shift_left(&mut self) {
        self.tape_ptr -= 1;
    }

    fn plus(&mut self) {
        self.tape[self.tape_ptr] = self.tape[self.tape_ptr].wrapping_add(1);
    }

    fn minus(&mut self) {
        self.tape[self.tape_ptr] = self.tape[self.tape_ptr].wrapping_sub(1);
    }

    fn comma(&mut self) {
        let mut input = String::new();
        let stdin = io::stdin();
        match stdin.read_line(&mut input) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error reading user input. Error: {}", err);
                process::exit(10);
            } 
        }

        if !input.is_ascii() {
            eprintln!("Input text should be in ascii format.");
            process::exit(11);
        }

        match input.as_bytes().get(0) {
            Some(c) => self.tape[self.tape_ptr] = *c,
            None => {
                eprintln!("No input detected.");
                process::exit(12);
            }
        }
    }
}

fn read_file() -> Vec<u8> {
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

    return source_text.into_bytes()
}

fn main() {

    let ascii_source: Vec<u8> = read_file();

    let source = Source {
        source_code: ascii_source,
        char_pointer: 0
    };

    let mut tape = Tape{
        source: source,
        tape: Box::new([0; 30000]),
        tape_ptr: 0,
    };

    while tape.source.peek() != None {
        match tape.source.read() {
            Some(b'+') => tape.plus(),
            Some(b'-') => tape.minus(),
            Some(b'>') => tape.shift_right(),
            Some(b'<') => tape.shift_left(),
            Some(b'.') => print!("{}", tape.tape[tape.tape_ptr] as char),
            Some(b',') => tape.comma(),
            Some(_) => (),
            None => println!("None.")
        }
    }
}
