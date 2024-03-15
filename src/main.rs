use std::{env, fs, io::Read};

fn main() {
    let args = env::args();
    let args_array: Vec<String> = args.skip(1).take(2).collect();
    if args_array.len() != 2 {
        println!("make sure to supply file path and memory size")
    } else {
        let filename = &args_array[0];
        let mem_size: i32 = args_array[1].parse().unwrap();

        let input = fs::read_to_string(filename).unwrap();

        println!("{}", interpret(&input, mem_size));
    }
}

fn interpret(input: &str, mem_size: i32) -> String {
    let mut ptr: i32 = 0;
    let mut memory: Vec<i32> = vec![0; mem_size as usize];
    let mut loop_indexes: Vec<i32> = Vec::new();
    let mut no_loop = false;
    let mut nested = 0;
    let mut output = String::new();

    let tokens = input.split("").collect::<Vec<&str>>();
    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i];
        if no_loop {
            if token == "]" {
                if nested != 0 {
                    nested -= 1;
                } else {
                    no_loop = false
                }
            } else if token == "[" {
                nested += 1;
            }
        } else {
            match token {
                "+" => {
                    memory[ptr as usize] += 1;
                    if memory[ptr as usize] > 255 {
                        memory[ptr as usize] = memory[ptr as usize] - 256;
                    }
                }
                "-" => {
                    memory[ptr as usize] -= 1;
                    if memory[ptr as usize] < 0 {
                        memory[ptr as usize] = 254 - memory[ptr as usize]
                    }
                }
                ">" => {
                    if ptr as usize > memory.len() - 2 {
                        ptr = 0;
                    } else {
                        ptr += 1;
                    }
                }
                "<" => {
                    if ptr < 1 {
                        ptr = (memory.len() - 1) as i32;
                    } else {
                        ptr -= 1;
                    }
                }
                "." => {
                    let ascii: u8 = memory[ptr as usize].try_into().unwrap();
                    output += &String::from(ascii as char);
                }
                "," => {
                    let byte = std::io::stdin().bytes().next().unwrap().unwrap();
                    memory[ptr as usize] = byte as i32;
                }
                "[" => {
                    if memory[ptr as usize] == 0 {
                        no_loop = true;
                    } else {
                        loop_indexes.push(i.try_into().unwrap());
                    }
                }
                "]" => {
                    i = *loop_indexes.last().unwrap() as usize - 1;
                    loop_indexes.pop();
                }

                _ => (),
            }
        }
        i += 1;
    }
    return output;
}
