use std::fs::File;
use std::io::{self, BufRead, BufReader};


fn main() -> io::Result<()> {
    let file = File::open("/home/ersan/AOC2023/day21/input.txt")?;
    let reader = BufReader::new(file);


    let mut starting_pos: (i32, i32) = (0, 0);

    let mut lines: Vec<Vec<char>> = reader
        .lines()
        .enumerate()
        .map(|(j, line)| {
            let mut new_line: Vec<char> = line.unwrap()
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if c == 'S' {
                        starting_pos = (i as i32, j as i32);
                    }
                    c
                })
            .collect();

            new_line.insert(0, '#');
            new_line.push('#');
            new_line
        })
    .collect();

    lines.insert(0, vec!['#'; lines[0].len()]);
    lines.push(vec!['#'; lines[0].len()]);

    let looping_values: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
    ];
    starting_pos = (starting_pos.0 + 1, starting_pos.1 + 1);


    let mut positions: Vec<(i32, i32)> = vec![starting_pos];

    let mut grid = lines.clone();
    let mut step_count = 0;
    let max_step = 26501365;
    let mut old = grid.clone();

    while max_step > step_count {
        for x in grid.iter() {
            for y in x.iter() {
                print!("{}", y);
            }
            println!();
        }
        let mut current_positions: Vec<(i32, i32)> = Vec::new();
        for pos in positions.iter() {
            for value in looping_values.iter() {
                let new_pos = (pos.0 + value.0, pos.1 + value.1);
                if grid[new_pos.1 as usize][new_pos.0 as usize] != '#' {
                    grid[new_pos.1 as usize][new_pos.0 as usize] = 'O';
                    if !current_positions.contains(&new_pos) {
                        current_positions.push(new_pos);
                    }
                }
            }        
            grid[pos.1 as usize][pos.0 as usize] = '.';
        }

        if compare(&grid, &reverse(old.clone())) {
            println!("looping");
            break;
        } 
        old = grid.clone();
        positions = current_positions.clone();

        step_count += 1;
    }

    let mut count = 0;
    for x in grid.iter() {
        for y in x.iter() {
            print!("{}", y);
            if *y == 'O' {
                count += 1;
            }
        }
        println!();
    }

    println!("count: {}", count);
    Ok(())
}

fn reverse(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for x in grid.iter_mut() {
        for y in x.iter_mut() {
            if *y == 'O' {
                *y = '.';
            } else if *y == '.' {
                *y = 'O';
            }
        }
    }
    grid
}

fn compare(grid1: &Vec<Vec<char>>, grid2: &Vec<Vec<char>>) -> bool {
    for y in 0..grid1.len() {
        for x in 0..grid1[y].len() {
            if grid1[y][x] != grid2[y][x] {
                return false;
            }
        }
    }
    true
}
