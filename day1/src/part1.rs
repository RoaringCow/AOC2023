use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {


    let file = File::open("/home/ersan/aoc2023/day1/input.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<i32>> = Vec::new();

    for line_result in reader.lines() {
        let mut line = line_result?;
        
        let mut line_numbers: Vec<i32> = Vec::new();

        for character in line.chars() {
            if character.is_numeric(){
                line_numbers.push(character as i32 - 48);
            }
        }

        lines.push(line_numbers);
    
    
    }

    let mut sum = 0;
    for line in lines {
        sum += line[0] * 10 + line[line.len() - 1];
    }
    println!("{}", sum);

    Ok(())
}
