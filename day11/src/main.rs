use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec;


fn main() -> io::Result<()> {
     
    let file = File::open("/home/ersan/AOC2023/day11/input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    
    let mut col_offsets: Vec<i64> = vec![0 ; lines[0].len()];
    let mut row_offsets: Vec<i64> = vec![0 ; lines.len()];

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

    for x in rows.iter() {
        row_offsets.iter_mut().enumerate().skip(x + 1).for_each(|(_, x)| *x += 999_999);
    }
    
    for y in cols.iter() {
        col_offsets.iter_mut().enumerate().skip(y + 1).for_each(|(_, x)| *x += 999_999);
    }

    
    
    let mut galaxies: Vec<(i64, i64)> = Vec::new();
    
    for (i, x) in lines.iter().enumerate() {
        for (j, y) in x.chars().enumerate() {
            if y == '#' {
                galaxies.push((i as i64, j as i64));
            }
        }
    }
    
    
    let mut distance_sum = 0;
    
    for x in galaxies.iter() {
        for y in galaxies.iter() {
            //println!("{:?}  {:?}  {}", x, y, (((x.0 + row_offsets[x.0 as usize]) - (y.0 + row_offsets[y.0 as usize])).abs() + ((x.1 + col_offsets[x.1 as usize]) - (y.1 + col_offsets[y.1 as usize])).abs()));
            distance_sum += ((x.0 + row_offsets[x.0 as usize]) - (y.0 + row_offsets[y.0 as usize])).abs() + ((x.1 + col_offsets[x.1 as usize]) - (y.1 + col_offsets[y.1 as usize])).abs();
        }
    }
    
    println!("{}", distance_sum / 2);
    
    
    Ok(())

}
