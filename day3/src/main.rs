use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_battery_packs() -> io::Result<Vec<String>> {
    let file = File::open("./data/battery_packs.jolts")?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut running_sum = 0;
    let battery_packs = get_battery_packs()?;
    for pack in battery_packs {
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
        running_sum += biggest_num;
    }
    println!("Running sum : {running_sum}");

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
