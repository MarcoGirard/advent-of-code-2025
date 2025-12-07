use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Operation {
    operands: Vec<u64>,
    operator: String,
}

impl Operation {
    fn new() -> Operation {
        Operation {
            operands: Vec::new(),
            operator: String::new(),
        }
    }
}

fn get_operations() -> io::Result<Vec<Operation>> {
    let file = File::open("./data/input_test.txt")?;

    let reader = BufReader::new(file);
    let input_grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut vals: Vec<char> = Vec::new();
            for c in line.chars() {
                vals.push(c);
            }
            vals
        })
        .collect();
    let mut operations: Vec<Operation> = Vec::new();
    let w = input_grid.len();
    let h = input_grid[0].len();
    let mut curr_op = Operation::new();
    for row in (0..h).rev() {
        let mut tmp_str = String::new();
        let mut only_space = true;
        for col in 0..w {
            let curr_char = input_grid[col][row];
            match curr_char {
                '+' | '*' => {
                    curr_op.operator = curr_char.to_string();
                    only_space = false
                }
                ' ' => {}
                _ => match curr_char.to_string().parse::<u64>() {
                    Ok(num) => {
                        tmp_str.push(curr_char);
                        only_space = false
                    }
                    Err(_) => {
                        eprintln!(
                            "You did not take that into account didn't you? {}",
                            curr_char
                        );
                    }
                },
            }
        }
        if only_space {
            operations.push(curr_op);
            curr_op = Operation::new();
        } else {
            curr_op.operands.push(tmp_str.parse::<u64>().unwrap());
            tmp_str = String::new();
        }
    }
    Ok(operations)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let operations = get_operations()?;
    let mut running_sum = 0;
    for op in operations {
        println!("{:?}", op.operands);
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
