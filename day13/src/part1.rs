use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("/home/ersan/AOC2023/day13/input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut line_group: Vec<String> = Vec::new();
    let mut big_group: Vec<Vec<String>> = Vec::new();

    for line in lines.iter() {
        if line == "" {
            big_group.push(line_group.clone());
            line_group.clear()
        } else {
            line_group.push(line.clone());
        }
        
    }
    

    let mut sum = 0;
    for group in big_group.iter(){
        let mut row_sum: i32 = function_that_does_things(&group);
        println!("rowsum");
        let mut flipped_group: Vec<String> = Vec::new();
        for x in 0..group[0].len() {
            let mut first_collumn: String = String::from("");
            for y in (0..group.len()).rev() {
                first_collumn.push_str(group[y].chars().nth(x).unwrap().to_string().as_str());
            }
            flipped_group.push(first_collumn);
        }

        // 0 1 2 3 4 5 6 7 8 9
        let mut col_sum: i32 = function_that_does_things(&flipped_group);
        println!("colsum");
        /*
        for x in group.iter(){
            println!("{:?}", x);
        }
        */
        println!("{} {}", col_sum, row_sum);
        sum += col_sum + row_sum * 100;
    }
    

    println!("{}", sum);

    Ok(())
}

fn function_that_does_things(vector: &Vec<String>) -> i32{
    let mut group = vector.clone();
    let mut sum = 0;
    for x in 0..group.len() - 1{
        //let mut difference = difference(&d)
        if group[x] == group[x + 1] {
            let mut is_complete = true;
            let mut i = 0;

            loop{
                println!("x: {}, i: {}, 1: {}  2: {}", x , i, x - i, x + 1 + i);
                if ((x as i32 - i as i32) < 0) || (x as i32 + 1 + i as i32 > group.len() as i32 - 1) {
                    break;
                }
                



                // difference değer zaten 1 ise yapma yoksa yap ve 1 arttır :)





                if group[x - i as usize] != group[x + 1 + i as usize] {
                    is_complete = false;
                }
                i += 1;
            }
            println!("{}", is_complete);
            if is_complete {
                sum += x + 1;
            }
        }
    }
    println!("{}", sum);

    sum as i32
}

fn difference(s1: &String, s2: &String) -> i32{
    let mut difference = 0;
    for x in 0..s1.len() {
        if s1.chars().nth(x).unwrap() != s2.chars().nth(x).unwrap() {
            difference += 1;
        }
    }
    //println!("str1: {} | str2: {} | num: {}", s1, s2, difference);
    difference
}
