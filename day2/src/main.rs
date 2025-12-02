use std::fs;
use strip_prefix_suffix_sane::StripPrefixSuffixSane;

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
            if repeats_twice(n) {
                total += n;
            }
        }
    }
    println!("Total sum of numbers that repeat twice: {}", total);
    Ok(())
}
