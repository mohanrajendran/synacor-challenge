use std::env;
use std::fs::File;
use std::io::Read;

fn read_file(file_name: &String, memory: &mut [u16]) -> std::io::Result<()>{
    let mut f = try!(File::open(file_name.to_string()));
    let mut buffer = Vec::new();

    let size = try!(f.read_to_end(&mut buffer))/2;

    for i in 0..size {
        let bit1 = buffer[i*2] as u16;
        let bit2 = buffer[i*2+1] as u16;

        let cell: u16 = bit1 + (bit2 << 8);

        memory[i] = cell;
    }
    
    Ok(())
}

fn fetch_and_execute(memory: &mut [u16], register: &mut [u16], stack: &mut Vec<u16>, pc: usize) -> Option<usize> {
    let op = memory[pc];

    if op == 0 {
        None
    } else if op == 19 {
        print!("{}", memory[pc+1] as u8 as char);
        Some(pc + 2)
    } else if op == 21 {
        Some(pc + 1)
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

    let mut memory: [u16; 32768] = [0; 32768];

    read_file(&args[1], &mut memory).expect("Unable to read program");

    let mut pc: usize = 0;
    let mut register: [u16; 8] = [0; 8];
    let mut stack: Vec<u16> = Vec::new();

    loop {
        pc = fetch_and_execute(&mut memory, &mut register, &mut stack, pc).expect("Halting code");
    }
}
