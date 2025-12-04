use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_rolls_map() -> io::Result<Vec<Vec<char>>> {
    let file = File::open("./data/paper_rolls.map")?;
    let reader = BufReader::new(file);
    let grid = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    Ok(grid)
}

fn count_around(rolls: &Vec<Vec<char>>, row: &usize, col: &usize) -> u8 {
    let row_i = *row as isize;
    let col_i = *col as isize;
    let mut count = 0;
    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let row_bounds = (0, rolls.len());
    let col_bounds = (0, rolls[0].len());
    for d in directions {
        let target_row = row_i + d.0;
        if (target_row as usize) < row_bounds.0 || (target_row as usize) >= row_bounds.1 {
            continue;
        }
        let target_col = col_i + d.1;
        if (target_col as usize) < col_bounds.0 || (target_col as usize) >= col_bounds.1 {
            continue;
        }
        if rolls[target_row as usize][target_col as usize] == '@' {
            count += 1;
        }
    }
    count
}

fn count_accessible_rolls(rolls: &mut Vec<Vec<char>>, sum: &mut u32) -> Result<(), &'static str> {
    let mut roll_count = 0;

    for row in 0..rolls.len() {
        for col in 0..rolls[0].len() {
            if rolls[row][col] == '.' || rolls[row][col] == 'x' {
                // print!("{}", rolls[row][col]);
                continue;
            }
            if count_around(&rolls, &row, &col) < 4 {
                // print!("x");
                roll_count += 1;
                rolls[row][col] = 'x';
            } else {
                // print!("{}", rolls[row][col]);
            }
        }
        // print!("\n");
    }
    if roll_count > 0 {
        *sum += roll_count;
        println!("{sum}");
        Ok(())
    } else {
        Err("No more")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rolls_map = get_rolls_map()?;
    let mut sum: u32 = 0;
    while count_accessible_rolls(&mut rolls_map, &mut sum).is_ok() {}
    println!("The number of accessible rolls is: {}", sum);
    Ok(())
}
