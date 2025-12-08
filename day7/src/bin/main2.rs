use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_input() -> io::Result<Vec<String>> {
    let mut args = env::args();
    args.next();
    let filename = args.next().unwrap_or("input".to_string());
    let filepath = format!("./data/{filename}.txt");
    println!("{filepath}");
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|line| line.unwrap()).collect();

    Ok(input)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input()?;
    // let mut beams: HashSet<usize> = HashSet::new();
    let mut beams: HashMap<usize, usize> = HashMap::new();
    for (i, c) in input[0].chars().enumerate() {
        if c == 'S' {
            beams.insert(i, 1);
        }
    }

    // we need to keep track of possibilities at all positions
    for line in 1..input.len() {
        let mut new_beams: HashMap<usize, usize> = HashMap::new();
        for (i, c) in input[line].chars().enumerate() {
            if let Some(beam_count) = beams.get(&i) {
                if c == '^' {
                    *new_beams.entry(i - 1).or_insert(0) += beam_count;
                    *new_beams.entry(i + 1).or_insert(0) += beam_count;
                    print!("^");
                } else {
                    *new_beams.entry(i).or_insert(0) += beam_count;
                    print!("|")
                }
            } else {
                print!(".");
            }
        }
        print!("\n");
        beams = new_beams;
    }
    let sum: usize = beams.values().sum();
    println!("{sum:?}");
    Ok(())
}
