use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("/home/ersan/AOC2023/day13/input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut line_group: Vec<Vec<char>> = Vec::new();
    let mut big_group: Vec<Vec<Vec<char>>> = Vec::new();

    for line in lines.iter() {
        if line.len() < 2 {
            big_group.push(line_group.clone());
            line_group.clear()
        } else {
            line_group.push(line.clone());
        }
        
    }
    

    let mut sum = 0;
    for group in big_group.iter(){
        let mut clone_group = group.clone();

        let row_sum = function_that_does_things(&mut clone_group);


        let mut flipped_group: Vec<Vec<char>> = Vec::new();
        for x in 0..clone_group[0].len() {
            let mut first_collumn: Vec<char> = Vec::new();
            for y in (0..clone_group.len()).rev() {
                first_collumn.push(clone_group[y][x]);
            }
            flipped_group.push(first_collumn);
        }
        
        println!("asd");
        // 0 1 2 3 4 5 6 7 8 9
        let col_sum = function_that_does_things(&mut flipped_group);
        /*
        for x in group.iter(){
            println!("{:?}", x);
        }
        */
        sum += col_sum + row_sum * 100;
        println!("bbbb");
    }
    

    println!("{}", sum);

    Ok(())
}

fn function_that_does_things(vector: &mut Vec<Vec<char>>) -> i32{
    let mut group = vector.clone();
    let mut sum = 0;
    for x in 0..group.len() - 1{
        let mut difference_value = 0;
        let mut i = 0;

        loop{
            difference_value += difference(&group[x - i as usize], &group[x + 1 + i as usize]);
            
            i += 1;
            //println!("x: {}  i: {} |  1: {}  2: {}   diff {}", x, i, x as i32 - i as i32, x + 1 + i as usize, difference_value);
            
            

            if ((x as i32 - i as i32) < 0) || (x as i32 + 1 + i as i32 > group.len() as i32 - 1) {
                break;
            }
        }

        if difference_value == 1 {
            println!("a: {}", x);
            sum += x + 1;
        }
    }
    println!("{}", sum);

    sum as i32
}

fn difference(s1: &Vec<char>, s2: &Vec<char>) -> i32{
    let mut difference = 0;
    for x in 0..s1.len() {
        if s1[x] != s2[x] {
            difference += 1;
        }
    }
    difference
}
