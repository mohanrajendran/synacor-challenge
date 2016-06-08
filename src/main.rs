use std::env;
use std::fs::File;
use std::io::Read;

fn read_file(file_name: &String, memory: &mut [usize]) -> std::io::Result<()>{
    let mut f = try!(File::open(file_name.to_string()));
    let mut buffer = Vec::new();

    let size = try!(f.read_to_end(&mut buffer))/2;

    for i in 0..size {
        let bit1 = buffer[i*2] as usize;
        let bit2 = buffer[i*2+1] as usize;

        let cell: usize = bit1 + (bit2 << 8);

        memory[i] = cell;
    }
    
    Ok(())
}

fn get_value(value: usize, register: &[usize]) -> usize {
    if value <= 32767 {
        value
    } else {
        register[(value-32768)]
    }
}

fn fetch_and_execute(memory: &mut [usize], register: &mut [usize], stack: &mut Vec<usize>, pc: usize) -> Option<usize> {
    let op = memory[pc];

    if op == 0 {
        None
    } else if op == 1 {
        let location = memory[pc+1] - 32768;
        let value = get_value(memory[pc+2], register);
        register[location] = value;
        Some(pc+3)
    } else if op == 2 {
        let value = get_value(memory[pc+1], register);
        stack.push(value);
        Some(pc+2)
    } else if op == 3 {
        let location = memory[pc+1] - 32768;
        match stack.pop() {
            Some(x) => {
                register[location] = x;
                Some(pc+2)
            }
            None => panic!("Popping an empty stack")
        }
    } else if op == 4 {
        let dest = memory[pc+1] - 32768;
        let value1 = get_value(memory[pc+2], register);
        let value2 = get_value(memory[pc+3], register);
        if value1 == value2 {
            register[dest] = 1;
        } else {
            register[dest] = 0;
        }
        Some(pc+4)
    } else if op == 5 {
        let dest = memory[pc+1] - 32768;
        let value1 = get_value(memory[pc+2], register);
        let value2 = get_value(memory[pc+3], register);
        if value1 > value2 {
            register[dest] = 1;
        } else {
            register[dest] = 0;
        }
        Some(pc+4)
    } else if op == 6 {
        let address = memory[pc+1];
        Some(get_value(address, register))
    } else if op == 7 {
        let value = get_value(memory[pc+1], register);
        if value != 0 {
            let dest = get_value(memory[pc+2], register);
            Some(dest)
        } else {
            Some(pc+3)
        }      
    } else if op == 8 {
        let value = get_value(memory[pc+1], register);
        if value == 0 {
            let dest = get_value(memory[pc+2], register);
            Some(dest)
        } else {
            Some(pc+3)
        }
    } else if op == 9 {
        let dest = memory[pc+1] - 32768;
        let value1 = get_value(memory[pc+2], register);
        let value2 = get_value(memory[pc+3], register);
        let sum = (value1 + value2) % 32768;
        register[dest] = sum;
        Some(pc+4)
    } else if op == 10 {
        let dest = memory[pc+1] - 32768;
        let value1 = get_value(memory[pc+2], register);
        let value2 = get_value(memory[pc+3], register);
        let sum = (value1 * value2) % 32768;
        register[dest] = sum;
        Some(pc+4)
    } else if op == 11 {
        let dest = memory[pc+1] - 32768;
        let value1 = get_value(memory[pc+2], register);
        let value2 = get_value(memory[pc+3], register);
        let sum = value1 % value2;
        register[dest] = sum;
        Some(pc+4)
    } else if op == 12 {
        let dest = memory[pc+1] - 32768;
        let value1 = get_value(memory[pc+2], register);
        let value2 = get_value(memory[pc+3], register);
        let result = value1 & value2;
        register[dest] = result;
        Some(pc+4)
    } else if op == 13 {
        let dest = memory[pc+1] - 32768;
        let value1 = get_value(memory[pc+2], register);
        let value2 = get_value(memory[pc+3], register);
        let result = value1 | value2;
        register[dest] = result;
        Some(pc+4)
     } else if op == 14 {
        let dest = memory[pc+1] - 32768;
        let value = get_value(memory[pc+2], register);
        let result = ((!(value as u16)) as usize) & 32767;        
        register[dest] = result;
        Some(pc+3)
    } else if op == 15 {
        let dest = memory[pc+1] - 32768;
        let source = memory[pc+2];
        register[dest] = memory[source];
        Some(pc+3)
    } else if op == 17 {
        stack.push(pc+2);
        let dest = get_value(memory[pc+1], register);
        Some(dest)
    } else if op == 19 {
        print!("{}", memory[pc+1] as u8 as char);
        Some(pc + 2)
    } else if op == 21 {
        Some(pc + 1)
    } else {
        panic!("Unknown op {} {} {} {}", op, memory[pc+1], memory[pc+2], memory[pc+3]);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./synacor-vm FILENAME");
        return;
    }

    let mut memory: [usize; 32768] = [0; 32768];

    read_file(&args[1], &mut memory).expect("Unable to read program");

    let mut pc: usize = 0;
    let mut register: [usize; 8] = [0; 8];
    let mut stack: Vec<usize> = Vec::new();

    loop {
        pc = fetch_and_execute(&mut memory, &mut register, &mut stack, pc).expect("Halting code");
    }
}
