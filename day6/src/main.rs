use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Operation {
    operands: Vec<String>,
    operator: String,
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
    let w = input_grid.len();
    let h = input_grid[0].len();
    for row in (0..h).rev() {
        for col in (0..w) {
            print!("{}", input_grid[col][row]);
        }
        print!("\n");
    }
    let operations: Vec<Operation> = Vec::new();
    Ok(operations)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let operations = get_operations()?;
    // let mut running_sum = 0;
    for op in operations {
        println!("{op:?}");
        // if op.operator == "*" {
        //     running_sum += op.operands.iter().fold(1, |acc, x| acc * x);
        // }
        // if op.operator == "+" {
        //     running_sum += op.operands.iter().fold(0, |acc, x| acc + x);
        // }
    }
    // println!("Running sum : {running_sum}");
    Ok(())
}
