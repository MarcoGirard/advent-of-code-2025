use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_battery_packs() -> io::Result<Vec<String>> {
    let file = File::open("./data/battery_packs.jolts")?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn get_biggest_jolts_2(pack: &str) -> u32 {
    let mut left = pack.chars().nth(0).unwrap().to_digit(10).unwrap();
    let mut right = pack.chars().nth(1).unwrap().to_digit(10).unwrap();
    let mut biggest_num = left * 10 + right;
    for i in 2..pack.len() {
        let curr_val_right = pack.chars().nth(i).unwrap().to_digit(10).unwrap();
        // check if should change right
        let conc_num = left * 10 + curr_val_right;
        if conc_num > biggest_num {
            biggest_num = conc_num;
            right = curr_val_right;
        }
        // check if should change left
        let curr_val_left = pack.chars().nth(i - 1).unwrap().to_digit(10).unwrap();
        let conc_num = curr_val_left * 10 + curr_val_right;
        if conc_num > biggest_num {
            biggest_num = conc_num;
            left = curr_val_left;
            right = curr_val_right;
        }
    }
    biggest_num
}

fn get_biggest_jolts_12(pack: &str) -> u128 {
    // keep track of taken indices to make sure we don't choose
    // the same multiple times
    let mut indices: [usize; 12] = [0; 12];
    let mut inner_iter_end = 0;

    for i in (0..12).rev() {
        let pack_index = pack.len() - i - 1;
        let mut biggest = pack.chars().nth(pack_index).unwrap().to_digit(10).unwrap();
        indices[11 - i] = pack_index;
        let mut furthest_biggest_index = pack_index;
        for j in (inner_iter_end..pack_index).rev() {
            let curr_val = pack.chars().nth(j).unwrap().to_digit(10).unwrap();
            if curr_val >= biggest {
                indices[11 - i] = j;
                biggest = curr_val;
            }
        }
        inner_iter_end = indices[11 - i] + 1;
    }
    let out = indices
        .iter()
        .map(|&i| format!("{}", pack.chars().nth(i).unwrap()))
        .collect::<Vec<_>>()
        .join("");
    out.parse().unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut running_sum: u128 = 0;
    let battery_packs = get_battery_packs()?;
    // if let pack = &battery_packs[0] {
    for pack in battery_packs {
        running_sum += get_biggest_jolts_12(&pack);
        // running_sum += get_biggest_jolts_12("987654321111111");
    }
    println!("The sum of the biggest jolts is: {}", running_sum);
    Ok(())
}
// pseudo-code
//
// left, right = strings[0:2]
// for char in strings[2:end] {
//     # would changing right make it bigger?
//         # yes -> right = char
//         # no -> continue
//     # Would changing left for i-1 make it bigger?
//         # yes -> left = strings[i-1]
//         #        right = strings[i]
//         # no -> continue
// }
