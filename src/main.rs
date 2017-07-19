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

    let mut register_pointer = 0usize;
    let mut code_pointer = 0usize;
    #[derive(Debug)]
    let mut code = Vec::new();
    let mut inloop_vec: Vec<usize> = Vec::new();
    let mut register: [i32;32] = [0;32];
    for c in content.chars() {
        code.push(c);
        //println!("{}", c);
    }

    //println!("{:?}", code);
    loop {
        match code[code_pointer] {
            '<' => if register_pointer != 0 {
                register_pointer -= 1;
            },
            '>' => register_pointer += 1,
            '+' => register[register_pointer] += 1,
            '-' => register[register_pointer] -= 1,
            '.' => print!("{}", to_ascii(&register[register_pointer])),
            '[' => inloop_vec.push(code_pointer),
            ']' => {
                if inloop_vec.is_empty() {
                    println!("ERROR: UNKNOWN LOOP CLOSING!");
                    break;
                } else if register[register_pointer] != 0 {
                    code_pointer = *inloop_vec.last().unwrap();
                } else {
                    inloop_vec.pop();
                }
            },
            _ => { },
        }
        code_pointer += 1;
        if code_pointer == code.len() {
            return
        }
        //println!("{:?}", register)
    }
    
}

fn to_ascii(i: &i32) -> String {
    match *i {
        x@0...127 => format!("{:?}", x as u8 as char),
        _ => "".into(),
    }
}

// print!("{}", to_ascii(&brainfuck_register[pointer]))