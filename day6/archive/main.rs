use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Operation {
    operands: Vec<u64>,
    operator: String,
}

fn get_operations() -> io::Result<Vec<Operation>> {
    let file = File::open("./data/input.txt")?;
    let mut reader = BufReader::new(file);
    let mut reader = reader
        .lines()
        .map(|line| line.expect("Failed to read line"));
    let mut operations: Vec<Operation> = reader
        .next()
        .unwrap()
        .split_whitespace()
        .map(|val| {
            let val = val.parse::<u64>().unwrap();
            Operation {
                operands: vec![val],
                operator: " ".to_string(),
            }
        })
        .collect();
    for (line_nb, line) in reader.enumerate() {
        for (i, val) in line.split_whitespace().enumerate() {
            let mut op: &mut Operation = operations.get_mut(i).expect("Should be there");
            match val.parse::<u64>() {
                Ok(num) => {
                    op.operands.push(num);
                }
                Err(_) => {
                    op.operator = val.to_string();
                }
            }
        }
    }
    Ok(operations)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let operations = get_operations()?;
    let mut running_sum = 0;
    for op in operations {
        if op.operator == "*" {
            running_sum += op.operands.iter().fold(1, |acc, x| acc * x);
        }
        if op.operator == "+" {
            running_sum += op.operands.iter().fold(0, |acc, x| acc + x);
        }
    }
    println!("Running sum : {running_sum}");
    Ok(())
}
