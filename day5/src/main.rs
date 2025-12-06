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

fn merge_ranges(ranges: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    /*
     *   if start < end_of_previous
     *       new_range = start_of_previous - end
     *   else
     *       range start_of_previous - end_of_previous
     * */

    let mut merged = vec![ranges[0]];
    for &(start, end) in &ranges[1..] {
        let last = merged.last_mut().unwrap();
        if start <= last.1 + 1 {
            // overlapping or contiguous
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }
    merged
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ranges = get_ranges()?;

    // Part one
    // let ids = get_ids()?;
    // let mut count: u32 = 0;
    // for id in &ids {
    //     println!("{id}");
    //     for (start, end) in &ranges {
    //         if id >= start && id <= end {
    //             count += 1;
    //             println!("ID {id} is in range {start}-{end}");
    //             break;
    //         }
    //     }
    // }
    // println!("{count}");

    // Part two
    let mut good_ids: HashSet<u128> = HashSet::new();

    ranges.sort_unstable_by_key(|&(start, _)| start);
    let merged_ranges = merge_ranges(ranges.clone());
    let max_len = ranges.len().max(merged_ranges.len());

    // for i in 0..max_len {
    //     let left = ranges
    //         .get(i)
    //         .map(|(x, y)| format!("({x: >15}, {y: >15})"))
    //         .unwrap_or_else(|| "-".into());
    //
    //     let right = merged_ranges
    //         .get(i)
    //         .map(|(x, y)| format!("({x: >15}, {y: >15})"))
    //         .unwrap_or_else(|| "-".into());
    //
    //     println!("{left} | {right}");
    // }
    //
    let mut nb_good_ids = 0;
    for (start, end) in merged_ranges {
        nb_good_ids += end - start + 1;
    }
    println!("Nb of good ids : {}", nb_good_ids);
    Ok(())
}
