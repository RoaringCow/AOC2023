use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {


    let file = File::open("/home/ersan/aoc2023/day1/input.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<i32>> = Vec::new();

    
    for line_result in reader.lines() {
        let mut line = line_result?;


        line = line.replace("one", mapdigit("one")).replace("two", mapdigit("two"))
            .replace("three", mapdigit("three")).replace("four", mapdigit("four"))
            .replace("five", mapdigit("five")).replace("six", mapdigit("six"))
            .replace("seven", mapdigit("seven")).replace("eight", mapdigit("eight"))
            .replace("nine", mapdigit("nine")).replace("zero", mapdigit("zero"));

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

fn mapdigit(s: &str) -> &str {
    match s {
        "one" => "o1e",
        "two" => "t2o",
        "three" => "t3e",
        "four" => "f4r",
        "five" => "f5e",
        "six" => "s6x",
        "seven" => "s7n",
        "eight" => "e8t",
        "nine" => "n9e",
        "zero" => "z0o",
        _ => " "
    }
}
