use std::fs::File;
use std::io::{self, BufRead, BufReader};



fn main() -> io::Result<()> {


    let file = File::open("/home/ersan/AOC2023/day4/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for (i, reader_line) in reader.lines().enumerate() {
        let line = reader_line?;


        let split = line[10..].split("|").collect::<Vec<&str>>();
        let winner: Vec<i32> = split[0]
            .split(" ")
            .filter_map(|s| s.parse().ok()) // Parse each substring to i32
            .collect();

        let current_hand: Vec<i32> = split[1]
            .split(" ")
            .filter_map(|s| s.parse().ok()) // Parse each substring to i32
            .collect();

        let mut score = 0;
        for number in current_hand {
            if winner.contains(&number) {
                score += 1;
            }
        }
        if score != 0 {
            sum += i32::pow(2, score - 1);
        }
    }

    println!("{}", sum);
    Ok(())
}
