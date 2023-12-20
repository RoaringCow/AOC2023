use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("/home/ersan/AOC2023/day14/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut boulders: Vec<(i32, i32)> = Vec::new();


    let mut lines: Vec<Vec<char>> = reader
        .lines().enumerate()
        .map(|(y ,line)| {
            line.unwrap()
                .chars().enumerate()
                .map(|(x, c)| {
                    if c == 'O' {
                        boulders.push((x as i32, y as i32));
                    }
                    c
                })
            .collect()
        })
    .collect();


    tilt(1, &mut lines, &boulders);

    for line in lines.iter() {
        println!("{}", line.iter().collect::<String>());
    }

}

fn tilt(direction: u8, lines: &mut Vec<Vec<char>>, boulders: &Vec<(i32, i32)>){

    let len = lines.len();
    let wid = lines[0].len();

    let mut any_moved = true;
    while any_moved {
        any_moved = false;
        for boulder in boulders.iter() {
            match direction {
                1 => {
                    if boulder.1 > 0 {
                        if lines[boulder.1 as usize - 1][boulder.0 as usize] == '.' {
                            lines[boulder.1 as usize - 1][boulder.0 as usize] = 'O';
                            lines[boulder.1 as usize][boulder.0 as usize] = '.';
                            any_moved = true;
                        }
                    }
                },
                2 => {
                    if boulder.0 < wid as i32 - 1 {
                        if lines[boulder.1 as usize][boulder.0 as usize + 1] == '.' {
                            lines[boulder.1 as usize][boulder.0 as usize + 1] = 'O';
                            lines[boulder.1 as usize][boulder.0 as usize] = '.';
                            any_moved = true;
                        }
                    }
                },
                3 => if boulder.1 < len as i32 - 1 {
                    if lines[boulder.1 as usize + 1][boulder.0 as usize] == '.' {
                        lines[boulder.1 as usize + 1][boulder.0 as usize] = 'O';
                        lines[boulder.1 as usize][boulder.0 as usize] = '.';
                        any_moved = true;
                    }
                },
                4 => if boulder.0 > 0 {
                    if lines[boulder.1 as usize][boulder.0 as usize - 1] == '.' {
                        lines[boulder.1 as usize][boulder.0 as usize - 1] = 'O';
                        lines[boulder.1 as usize][boulder.0 as usize] = '.';
                        any_moved = true;
                    }
                },
                _ => panic!("Invalid direction"),
            }
        }

        for line in lines.iter() {
            println!("{}", line.iter().collect::<String>());
        }
        println!("any_moved: {}\n", any_moved);




    } 


}
