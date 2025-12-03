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
        // if let Some(pack) = battery_packs.first() {
        // let pack = "242625254243433385231332255231";
        // {
        let mut left = pack.chars().nth(0).unwrap().to_digit(10).unwrap();
        let mut right = pack.chars().nth(1).unwrap().to_digit(10).unwrap();
        let mut biggest_num = left * 10 + right;
        // println!("{}", biggest_num);
        for i in 2..pack.len() {
            // println!(" atpos {i}");
            let curr_val_right = pack.chars().nth(i).unwrap().to_digit(10).unwrap();
            // check if should change right
            let conc_num = left * 10 + curr_val_right;
            // println!("Comparing pot: {}, biggest {}", conc_num, biggest_num);
            if conc_num > biggest_num {
                // println!("binggest becomes {conc_num}");
                biggest_num = conc_num;
                // println!("Right becomes {curr_val_right}");
                right = curr_val_right;
            }
            // check if should change left
            let curr_val_left = pack.chars().nth(i - 1).unwrap().to_digit(10).unwrap();
            let conc_num = curr_val_left * 10 + curr_val_right;
            // println!("Checking left : pot {conc_num}, curr_big {biggest_num}");
            if conc_num > biggest_num {
                // println!("binggest becomes {conc_num}");
                biggest_num = conc_num;
                // println!("left becomes {curr_val_left}");
                left = curr_val_left;
                // println!("Right becomes {curr_val_right}");
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
