use std::io::{BufRead, BufReader};
use std::fs::File;


pub fn main() {
    let f = BufReader::new(File::open("/home/ersan/AOC2023/day15/input.txt").expect("open failed"));

    let lines: Vec<Vec<char>> = f
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    

    let binding = &lines[0].clone().into_iter().collect::<String>();
    let big_lens_vector: Vec<&str>  = binding.split(',').collect::<Vec<&str>>();


    // GİT KENDİ VERİ TİPİNİ YAP STUCT İMPL FALAN
    
    
    // part1

    let mut sum: i128 = 0;
    for current_lens in big_lens_vector.iter(){
        sum += hash(current_lens);
    }

    println!("part1: {}", sum);

    /*
    println!("{}", hash("kjkt"));
    */


    for current_lens in big_lens_vector.iter(){
       
    }
}

fn hash(value: &str) -> i128 {
    let mut sum: i128= 0;
    for character in value.chars() {
        sum = ((character as i128 + sum) * 17 as i128) % 256 as i128;
    }
    sum
}


