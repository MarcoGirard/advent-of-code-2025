use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_ranges() -> io::Result<Vec<(u128, u128)>> {
    let file = File::open("./data/ranges.txt")?;
    let reader = BufReader::new(file);
    // let (start, end) = "123-456".split_once('-').unwrap();
    let ranges = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (start, end) = line.split_once("-").expect("invalid range");
            let start = start.parse::<u128>().expect("can't parse");
            let end = end.parse::<u128>().expect("can't parse");
            (start, end)
        })
        .collect();
    Ok(ranges)
}

fn get_ids() -> io::Result<HashSet<u128>> {
    let file = File::open("./data/ids.txt")?;
    let reader = BufReader::new(file);
    let ids = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u128>().unwrap()) // safe if data is clean
        .collect();
    Ok(ids)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ranges = get_ranges()?;
    let ids = get_ids()?;

    // Part one
    let mut count: u32 = 0;
    for id in &ids {
        println!("{id}");
        for (start, end) in &ranges {
            if id >= start && id <= end {
                count += 1;
                println!("ID {id} is in range {start}-{end}");
                break;
            }
        }
    }
    println!("{count}");
    Ok(())
}
