use std::fs;
use strip_prefix_suffix_sane::StripPrefixSuffixSane;

fn repeats_n_times(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    for size in 1..=(len / 2) {
        // check if len(s) is divisible by size
        if len % size != 0 {
            continue;
        }
        // groups by size
        let groups: Vec<&str> = s
            .as_bytes()
            .chunks(size)
            .map(|c| std::str::from_utf8(c).unwrap())
            .collect();

        if groups.windows(2).all(|w| w[0] == w[1]) {
            return true;
        } else {
            continue;
        }
    }
    false
}

fn repeats_twice(n: u64) -> bool {
    let s = n.to_string();
    if s.len() % 2 != 0 {
        return false;
    }
    let mid = s.len() / 2;
    let first_half = &s[..mid];
    let second_half = &s[mid..];
    first_half == second_half
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "./data/input.txt";
    let contents = fs::read_to_string(filename)?;
    let ranges = contents.split(",");
    let mut total: u64 = 0;
    for range in ranges {
        let bounds = range.split("-").collect::<Vec<&str>>();
        let start: u64 = bounds[0].strip_suffix_sane("\n").parse().unwrap();
        let end: u64 = bounds[1].strip_suffix_sane("\n").parse().unwrap();
        for n in start..=end {
            if repeats_n_times(n) {
                total += n;
            }
        }
    }
    println!("Total sum of numbers that repeat twice: {}", total);
    Ok(())
}
