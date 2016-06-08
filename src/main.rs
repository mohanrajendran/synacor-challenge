use std::env;
use std::fs::File;
use std::io::Read;

fn read_file(fileName: &String) -> std::io::Result<Vec<u16>>{
    let mut f = try!(File::open(fileName.to_string()));
    let mut buffer = Vec::new();

    let size = try!(f.read_to_end(&mut buffer))/2;

    let mut result: Vec<u16> = Vec::new();

    for i in 0..size {
        let bit1 = buffer[i*2] as u16;
        let bit2 = buffer[i*2+1] as u16;

        let bit: u16 = bit1 + (bit2 << 8);

        result.push(bit);
    }
    
    Ok(result)
}

fn execute(program: &Vec<u16>, pc: usize) -> Result<usize,usize> {
    let op = program[pc];

    if op == 0 {
        Err(pc) 
    } else if op == 19 {
        print!("{}", program[pc+1] as u8 as char);
        Ok(pc + 2)
    } else if op == 21 {
        Ok(pc + 1)
    } else {
        panic!("Unknown op {}", op);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./synacor-vm FILENAME");
        return;
    }

    let program = read_file(&args[1]).unwrap();

    let mut pc: usize = 0;

    while true {
        pc = execute(&program, pc).unwrap();
    }
}