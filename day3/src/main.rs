use std::fs::File;
use std::io::{self, BufRead, BufReader};



fn main() -> io::Result<()> {


    let file = File::open("/home/ersan/AOC2023/day3/input.txt")?;
    let reader = BufReader::new(file);

    let mut numlines: Vec<Vec<char>>= Vec::new();

    for (j, line_result) in reader.lines().enumerate() {
        let mut line = line_result?;
        let mut nums: Vec<char> = Vec::new();

        for character in line.chars(){
            nums.push(character);
        }
        numlines.push(nums);

    }

    
    
    let mut modified = numlines.clone();
    
    for (i, line) in numlines.iter().enumerate() {
        for (j, character) in line.iter().enumerate() {
            if character.is_numeric() {
                let mut is_there = false;
                if !check_around(j as i32, i as i32, &numlines).0 {
                    let (right, isnumber, a) = check_around(j as i32 + 1, i as i32, &numlines);

                    if !right && isnumber {
                        
                        let (right, isnumber, a) = check_around(j as i32 + 2, i as i32, &numlines);
                        if isnumber {
                            is_there = right;
                        }

                    } else {
                        is_there = right && isnumber;
                    }
                    
                    if !is_there {
                        let (left, isnumber, a) = check_around(j as i32 - 1, i as i32, &numlines);

                        if !left && isnumber {

                            let (left, isnumber, a) = check_around(j as i32 - 2, i as i32, &numlines);
                            if isnumber {
                                is_there = left;
                            }
    
                        } else {
                            is_there = left && isnumber;
                        }

                    }

                } else {
                    is_there = true;
                } 
                    
                if !is_there {
                    modified[i][j] = ' ';
                }
            } else {
                modified[i][j] = ' ';
            }
        }
    }

    
    let mut sum = 0;

    let mut stripped: Vec<Vec<String>> = Vec::new();

    for y in &modified {
        let y_str: String = y.into_iter().collect();
        let words: Vec<String> = y_str.split_whitespace().map(String::from).collect();
        stripped.push(words.clone());
        for word in words.clone() {
            //print!("{} ", word);
            if let Ok(number) = word.parse::<i32>() {
                sum += number;
            }
        }
        //println!();
        //println!("      {}   {}", sum,  exsum);
    }
    println!("part1: {}", sum);



    let mut finalvec: Vec<Vec<String>> = Vec::new()
    ;
    for (i, line) in modified.iter().enumerate() {
        let mut is_number = false;
        let mut index = 0;
        let mut subfinalvec: Vec<String> = Vec::new();

        for (j, character) in line.iter().enumerate() {
            if character.is_numeric() {
                subfinalvec.push(stripped[i][index].clone());
                is_number = true;
            }else {
                if numlines[i][j] == '*' {
                    subfinalvec.push("*".to_string());
                } else{
                    subfinalvec.push(" ".to_string());
                }
                    if is_number {
                        index += 1;
                    }
                    is_number = false;
            }
        }
        finalvec.push(subfinalvec);
    }


    let mut sum = 0;
    for i in 0..finalvec.len(){
        for j in 0..finalvec[0].len() {

            if finalvec[i][j] == '*'.to_string() {
                let mut nums: Vec<i32> = Vec::new();
                for y in 0..3 {
                    for x in 0..3 {
                        let row = (i as i32 + y as i32 - 1).clamp(0, (finalvec.len() - 1) as i32) as usize;
                        let col = (j as i32 + x as i32 - 1).clamp(0, (finalvec[0].len() - 1) as i32) as usize;
                        if let Ok(num) = finalvec[row][col].parse::<i32>() {
                            nums.push(num);
                        }
            
                    }
                }
                finalvec[i][j] = " ".to_string();
                let mut nodupe = Vec::new();
                nums.retain(|&x| {
                    if nodupe.contains(&x) {
                        false
                    } else {
                        nodupe.push(x);
                        true
                    }
                });
                if nums.len() == 2 {
                    sum += nums[0] * nums[1];

                }
            }
        }
    }

    println!("part2: {}", sum);
    Ok(())
}


fn check_around(j: i32, i: i32, lines: &Vec<Vec<char>>) -> (bool, bool, char){
    let mut is_there = false;
    for y in 0..3 {
        for x in 0..3 {
            let row = (i as i32 + y as i32 - 1).clamp(0, (lines.len() - 1) as i32) as usize;
            let col = (j as i32 + x as i32 - 1).clamp(0, (lines[0].len() - 1) as i32) as usize;
            
            if lines[row][col] != '.' && !lines[row][col].is_numeric(){
                is_there = true;
            }

        }
    }
    let row = i.clamp(0, (lines.len() - 1) as i32) as usize;
    let col = j.clamp(0, (lines[0].len() - 1) as i32) as usize;
    return (is_there, lines[row][col].is_numeric(), lines[row][col]);
}