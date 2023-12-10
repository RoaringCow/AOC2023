use std::fs::File;
use std::io::{self, BufRead, BufReader};


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
    

    let mut cleared_map: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; lines.len()];

    let mut pipe_lenght = 0;

    // 1 up, 2 right, 3 down, 4 left
    // i looked up the puzzle to find the s type. i will make another system later on.
    let mut direction = 3;
    let mut current_pos = starting_pos;
    
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
        cleared_map[current_pos.1][current_pos.0] = current_pipe;

        if current_pos == starting_pos {
            break;
        }
    }
    let mut count = 0;
    
    cleared_map[starting_pos.1][starting_pos.0] = '7';
    
    cleared_map.push(vec!['.'; cleared_map.len()]);

    let mut big_map = make_the_array_go_big(&cleared_map);
    
    fill_2darray((1, 1), &mut big_map, ' ');
    
    for (_i, y) in big_map.iter().enumerate(){
        for (_j, x) in y.iter().enumerate(){
            if *x == '.' {
                count += 1;
                //print!("\x1b[46m");
            }
            //print!("{}\x1b[0m", x);
            
        }
        //println!();
    }
    
    println!("part1: {}, part2: {}", pipe_lenght / 2, count);


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

fn make_the_array_go_big(array: &Vec<Vec<char>>) -> Vec<Vec<char>>{
    let mut new_array: Vec<Vec<char>> = vec![vec!['*'; array[0].len() * 3]; array.len() * 3];

    for (i, line) in array.iter().enumerate() {
        for (j, character) in line.iter().enumerate() {
            let i = i * 3;
            let j = j * 3;
            new_array[i][j] = *character;
            match character {
                '|' => {new_array[i + 1][j] = '|'; new_array[i - 1][j] = '|';},
                '-' => {new_array[i][j + 1] = '-'; new_array[i][j - 1] = '-';},
                'L' => {new_array[i - 1][j] = '|'; new_array[i][j + 1] = '-';},
                'J' => {new_array[i - 1][j] = '|'; new_array[i][j - 1] = '-';},
                '7' => {new_array[i + 1][j] = '|'; new_array[i][j - 1] = '-';},
                'F' => {new_array[i + 1][j] = '|'; new_array[i][j + 1] = '-';},
                '.' => {
                    for y in 0..3 {
                        for x in 0..3 {
                            if (i > 1 && j > 1 && i > array.len() - 1 && j > line.len() - 1) && (x != 1 && y != 1){
                                new_array[i - 1 + y][j - 1 + x] = '*';
                            }
                            
                        }
                    }
                },
                _ => (),
            }
        }
    }

    new_array
}

fn fill_2darray(starting_position: (usize, usize), array: &mut Vec<Vec<char>>, new_value: char){
    let mut queue: Vec<(usize, usize)> = vec!(starting_position);
    
    //let value_to_be_changed = array[starting_position.1][starting_position.0];
    
    while let Some((row, col)) = queue.pop() {
        if array.len() > row && array[0].len() > col {
            if array[row][col] == '.' || array[row][col] == '*' {
                array[row][col] = new_value;
                
                queue.push((row + 1, col));
                queue.push((row - 1, col));
                queue.push((row, col - 1));
                queue.push((row, col + 1));
            }
        }
    }

}