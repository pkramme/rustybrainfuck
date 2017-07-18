use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;

fn main() {
    println!("Rusty Brainfuck Interpreter");
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let mut file = match File::open(&path) {
        Err(why) => panic!("{}", why.description()),
        Ok(file) => file, // if no error, return fd
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("{}", why.description()),
        Ok(_) => {},
    };
    //println!("{}", content);
    // Now that the file is loaded, lets create the array for brainfuck
    
    let mut brainfuck_register: [i32;32] = [0;32];
    let mut pointer = 0;

    for c in content.chars() {
        if c == '>' {
            pointer = pointer + 1;
            //println!(">")
        } else if c == '<' {
            pointer = pointer - 1;
            //println!("<")
        } else if c == '+' {
            brainfuck_register[pointer] = brainfuck_register[pointer] + 1;
            //println!("+")
        } else if c == '-' {
            brainfuck_register[pointer] = brainfuck_register[pointer] - 1;
            //println!("-")
        } else if c == ',' {
            print!("{}", to_ascii(&brainfuck_register[pointer]))
        }
    }
    //println!("{:?}", brainfuck_register)
    
}

fn to_ascii(i: &i32) -> String {
    match *i {
        x@0...127 => format!("{:?}", x as u8 as char),
        _ => "".into(),
    }
}