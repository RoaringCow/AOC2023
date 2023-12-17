use std::fs::File;
use std::io::{self, BufRead, BufReader};
#[allow(unused_imports)]
use std::{thread, time};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Beam {
    pos: (i32, i32),
    direction: u8,
    // 1 is up
    // 2 is right
    // 3 is down
    // 4 is left
}

impl Beam {
    pub fn new(pos: (i32, i32), direction: u8) -> Beam{
        Beam { pos, direction}
    }


    pub fn move_beam(&mut self, grid: &mut Vec<Vec<Box>>) -> Vec<Beam> {
        let mut new_beam: Vec<Beam> = vec![];

        match self.direction {
            1 => self.pos.1 -= 1,
            2 => self.pos.0 += 1,
            3 => self.pos.1 += 1,
            4 => self.pos.0 -= 1,
            _ => ()
        }

        // some cheating
        if self.is_in(&grid) {

            grid[self.pos.1 as usize][self.pos.0 as usize].energized = true;

            match grid[self.pos.1 as usize][self.pos.0 as usize].tile_type {
                '/' => {
                    self.direction = match self.direction {
                        1 => 2,
                        2 => 1,
                        3 => 4,
                        4 => 3,
                        _ => self.direction
                    }
                },
                '\\' => {
                    self.direction = match self.direction {
                        1 => 4,
                        2 => 3,
                        3 => 2,
                        4 => 1,
                        _ => self.direction
                    }
                },
                '|' => {
                    self.direction = match self.direction {
                        2 =>{
                            new_beam.push(Beam::new(self.pos, 3));
                            1
                        },
                        4 => {
                            new_beam.push(Beam::new(self.pos, 3));
                            1
                        },
                        _ => self.direction
                    }
                },
                '-' => {
                    self.direction = match self.direction {
                        1 => {
                            new_beam.push(Beam::new(self.pos, 4));
                            2
                        },
                        3 => {
                            new_beam.push(Beam::new(self.pos, 4));
                            2
                        },
                        _ => self.direction
                    }
                },
                _ => (),
            }

        }
    new_beam
} 

    pub fn is_in(&self, grid: &Vec<Vec<Box>>) -> bool{
        if self.pos.0 >= 0 && self.pos.0 < grid[0].len() as i32 && self.pos.1 >= 0 && self.pos.1 < grid.len() as i32{
            return true;
        }
        false
    }
}

#[derive(Debug)]
struct Box {
    tile_type: char,
    energized: bool,
}



fn main() -> io::Result<()> {
    let file = File::open("/home/ersan/AOC2023/day16/input.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<Box>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| Box {
                    tile_type: c,
                    energized: false,
                })
                .collect()
        })
        .collect();


    let mut beams: Vec<Beam> = vec![Beam::new((0, 0), 2)];
    
    for _ in 0..30 {
        for x in 0..beams.len() {
            let beam_to_add = beams[x].move_beam(&mut lines);
            beams.extend(beam_to_add);
        }
        beams.retain(|x| x.is_in(&lines));
        
        let mut energized_count = 0;
        for x in lines.iter() { 
            for y in x.iter() {
                if y.energized {
                    //print!("#");
                    energized_count += 1;
                }else {
                    //print!(".");
                }
            }
           // println!();
        }
        println!("{}", energized_count);
        
        //thread::sleep(ten_millis);

    }


    Ok(())
}
