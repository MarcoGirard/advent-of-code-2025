use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_input() -> io::Result<Vec<String>> {
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|line| line.unwrap()).collect();

    Ok(input)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input()?;
    let mut beams: HashSet<usize> = HashSet::new();
    for (i, c) in input[0].chars().enumerate() {
        if c == 'S' {
            beams.insert(i);
        }
    }
    let mut possibilities: usize = 0;
    for line in 1..input.len() {
        for (i, c) in input[line].chars().enumerate() {
            if c == '^' && beams.contains(&i) {
                beams.remove(&i);
                beams.insert(i - 1);
                beams.insert(i + 1);
                split_count += 1;
            }
        }
        possibilities *= beams.len() - beams_len; // that's how many were added on that line
        beams_len = beams.len();
    }
    println!("{possibilities}");
    Ok(())
}
