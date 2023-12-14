use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("/home/ersan/AOC2023/day12/input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    for line in lines.iter() {

        let split: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        
        
        let numbers: Vec<i32> = split[1]
            .split(',')
            .map(|s| s.parse().expect("parse error"))
            .collect();
        
            let max_num = numbers.iter().max().cloned().unwrap_or_default();

        let part_groups: Vec<String> = split[0]
            .split('.')
            .filter(|s| !s.is_empty() && s.len() >= max_num as usize)
            .map(|s| s.to_string())
            .collect();
        
        println!("{:?} {:?}", part_groups, numbers);
        
        for (i, group) in part_groups.iter().enumerate() {
            let count = group.chars().filter(|&c| c == '#').count();
            
            println!("{}", (1..numbers[i] - count as i32).product::<i32>() );
        }


        /*
        P(n,k)=(n−k)!n!​
         */

    }

    Ok(())
}
