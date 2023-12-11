use std::fs::File;
use std::io::{self, BufRead, BufReader};


fn main() -> io::Result<()> {
     
    let file = File::open("/home/ersan/AOC2023/day11/input.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    
    
    let mut cols: Vec<usize> = Vec::new();
    let mut rows: Vec<usize> = Vec::new();

    for (i ,line) in lines.iter().enumerate() {
        if !line.contains("#"){
            rows.push(i);
        }
    }

    for x in 0..lines[0].len() {
        let mut contains = false;
        for y in 0..lines.len() {
            if lines[y].chars().nth(x) == Some('#') {
                contains = true;
            }
        }
        if !contains {
            cols.push(x);
        }
    }

    for (i, x) in rows.iter().enumerate() {
        lines.insert(*x + i, ".".repeat(lines[0].len()))
    }
    
    for (i, y) in cols.iter().enumerate() {
        for x in 0..lines.len() {
            lines[x].insert(*y + i, '.');
        }
    }

    let mut galaxies: Vec<(i32, i32)> = Vec::new();

    for (i, x) in lines.iter().enumerate() {
        for (j, y) in x.chars().enumerate() {
            if y == '#' {
                galaxies.push((i as i32, j as i32));
            }
            print!("{}", y);
        }
        println!();
    }
    println!();
    

    let mut distance_sum = 0;

    for x in galaxies.iter() {
        for y in galaxies.iter() {
            //println!("{:?}  {:?}  {}", x, y, (x.0 - y.0).abs()  + (x.1 - y.1).abs() - 1);
            distance_sum += (x.0 - y.0).abs() + (x.1 - y.1).abs();
        }
    }

    println!("part1: {}", distance_sum / 2);

    Ok(())

}
