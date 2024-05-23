use std::{env, io};
use std::io::Write;
mod header_elf;
mod x64;
mod x32;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let filepath = args[1].as_str();
        match header_elf::read_elf_headers(filepath) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("{e}");
            }
        };
    }else {
        loop {
            let mut filepath = String::new();
            print!("enter the path of elf : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut filepath).unwrap();
            let filepath = filepath.trim().trim_matches('"');
            match header_elf::read_elf_headers(filepath) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{e}");
                }
            };
        }
    }
}
