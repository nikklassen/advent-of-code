use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn get_value(x: &str, registers: &HashMap<&str, i64>) -> i64 {
    x.parse::<i64>()
        .ok()
        .or(registers.get(x).map(|i| *i))
        .or(Some(0))
        .unwrap()
}

fn get_instructions() -> Vec<Vec<String>> {
    let file = File::open("input").unwrap();
    BufReader::new(file).lines().map(|line| {
        line.unwrap().split(" ").map(str::to_owned).collect()
    }).collect()
}

fn main() {
    let instructions = get_instructions();
    let mut registers: HashMap<&str, i64> = HashMap::new();

    let mut ip: i64 = 0;
    let mut num_mul = 0;
    let default_value = "0".to_string();
    loop {
        if ip < 0 || ip >= (instructions.len() as i64) {
            break;
        }

        let instruction: &Vec<_> = &instructions[ip as usize];
        let x = &instruction[1];
        let x_value = get_value(x, &registers);
        let y_value = get_value(&instruction.get(2).unwrap_or(&default_value), &registers);
        match instruction[0].as_str() {
            "set" => {
                registers.insert(x, y_value);
            },
            "sub" => {
                registers.insert(x, x_value - y_value);
            },
            "mul" => {
                num_mul += 1;
                registers.insert(x, x_value * y_value);
            },
            "jnz" => {
                if x_value != 0 {
                    ip += y_value;
                } else {
                    ip += 1;
                }
            },
            _ => unreachable!(),
        }
        if instruction[0].as_str() != "jnz" {
            ip += 1;
        }
    }
    println!("{}", registers.get("h").unwrap());
}