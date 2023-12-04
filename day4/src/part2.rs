use std::fs::File;
use std::io::{self, BufRead, BufReader};



fn main() -> io::Result<()> {


    let file = File::open("/home/ersan/AOC2023/day4/input.txt")?;
    let reader = BufReader::new(file);

    let mut cards: Vec<String> = Vec::new();
    let mut card_counts: Vec<i32> = Vec::new();
    for reader_line in reader.lines() {
        let line = reader_line?;

        cards.push(line.to_string());
        card_counts.push(1);
    }

    for card in cards.iter() {

        let card_number= card[4..8].trim().parse::<i32>().unwrap();
            
        //print!("{:?}  ", card_number);

        let split = card[10..].split("|").collect::<Vec<&str>>();
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
        for _ in 0..card_counts[(card_number - 1) as usize] {

            for x in 0..score {
                card_counts[(card_number + x) as usize] += 1;
                //print!("{} ", 1 + x + card_number);
            }
        }
        //println!("{}", card_counts[(card_number - 1) as usize]);
    }
    let mut sum = 0;

    for x in card_counts.iter(){
        sum += x;
        //println!("{}", x);
    }
    println!("{}", sum);
    
    Ok(())
}
