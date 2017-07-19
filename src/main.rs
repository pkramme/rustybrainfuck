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

    let mut instruction_pointer = 0usize;
    let mut tape_pointer = 0usize;
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
        match code[tape_pointer] {
            '<' => if instruction_pointer != 0 {
                instruction_pointer -= 1;
            },
            '>' => instruction_pointer += 1,
            '+' => register[instruction_pointer] += 1,
            '-' => register[instruction_pointer] -= 1,
            '.' => print!("{}", to_ascii(&register[instruction_pointer])),
            '[' => inloop_vec.push(tape_pointer),
            ']' => {
                if inloop_vec.is_empty() {
                    println!("ERROR: UNKNOWN LOOP CLOSING!");
                    break;
                } else if register[instruction_pointer] != 0 {
                    tape_pointer = *inloop_vec.last().unwrap();
                } else {
                    inloop_vec.pop();
                }
            },
            _ => { },
        }
        tape_pointer += 1;
        if tape_pointer == code.len() {
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