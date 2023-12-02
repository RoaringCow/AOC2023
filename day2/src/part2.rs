use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {


    let file = File::open("/home/ersan/AOC2023/day2/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum_power = 0;

    for (j, line_result) in reader.lines().enumerate() {
        let mut line = line_result?;

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        // Remove the Game x: part
        let first_split = line.split(": ").collect::<Vec<&str>>();
        line = first_split[1].to_string();

        let sets = line.split(";").collect::<Vec<&str>>();

        for (_i, set) in sets.iter().enumerate() {
            let mut parts = set.split(",").collect::<Vec<&str>>();

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for part in parts.iter_mut() {
                part = part.trim();
                let splitted = part.split(" ").collect::<Vec<&str>>();
                if part.contains("red") {
                    red += splitted[0].parse::<i32>().unwrap();
                }
                if part.contains("green") {
                    green += splitted[0].parse::<i32>().unwrap();
                }
                if part.contains("blue") {
                    blue += splitted[0].parse::<i32>().unwrap();
                }
            }
            if red > max_red {max_red = red;}
            if green > max_green {max_green = green;}
            if blue > max_blue {max_blue = blue;}
            //println!("Game {}: set{} (red: {} green: {} blue: {})", j + 1, i + 1, max_red, max_green, max_blue);
        }
        sum_power += max_red * max_green * max_blue;
    }
    println!("{}", sum_power);
    Ok(())
}