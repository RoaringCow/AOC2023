use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::{vec, usize};


fn main() -> io::Result<()> {
    
    
    
    
    let file = File::open("/home/ersan/AOC2023/day10/input.txt")?;
    let reader = BufReader::new(file);
    
    let mut lines: Vec<Vec<char>> = Vec::new();
    
    let mut starting_pos: (usize, usize) = (1, 1);
    
    
    
    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        
        lines.push(line.chars().collect::<Vec<char>>());
        
        let s_index = line.find("S");
        if !s_index.is_none() {
            starting_pos = (s_index.unwrap(), i);
        }
    }
    println!("{:?}  {}", starting_pos, lines[starting_pos.1][starting_pos.0]);
    
    
    let mut pipe_lenght = 0;
    
    
    // 1 up, 2 right, 3 down, 4 left
    // i looked up the puzzle to find the s type. i will make another system later on.
    let mut direction = 3;
    let mut current_pos = starting_pos;

    //println!("{:?}", distances);
    
    loop {
        match direction {
            1 => current_pos.1 -= 1,
            2 => current_pos.0 += 1, 
            3 => current_pos.1 += 1,
            4 => current_pos.0 -= 1,
            _ => eprintln!("Büyük bir sorun var")
        }
    
        let current_pipe = lines[current_pos.1][current_pos.0];
        
        direction = match current_pipe {
            // just changing the direction.
            // If you look at how the pipes are shaped you will get the numbers.
            'L' => if direction == 3 { 2 } else { 1 },
            'J' => if direction == 3 { 4 } else { 1 },
            '7' => if direction == 2 { 3 } else { 4 },
            'F' => if direction == 4 { 3 } else { 2 },
            _ => direction,
        };
        
        pipe_lenght += 1;
        println!("{:?}", current_pos);

        if current_pos == starting_pos {
            break;
        }
    }
    println!("{}", pipe_lenght / 2);

    
    
    

    /*
    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    */
    
    Ok(())
}


