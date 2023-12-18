use std::fs::File;
use std::io::{self, BufRead, BufReader};


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




        if !self.is_in(grid) {
            return new_beam;
        }
        
        if grid[self.pos.1 as usize][self.pos.0 as usize].energized.1.contains(&self.direction) {
            return new_beam;
        }

        grid[self.pos.1 as usize][self.pos.0 as usize].energized.0 = true;
        grid[self.pos.1 as usize][self.pos.0 as usize].energized.1.push(self.direction);

        match grid[self.pos.1 as usize][self.pos.0 as usize].tile_type {
            '/' => {
                let direction = match self.direction {
                    1 => 2,
                    2 => 1,
                    3 => 4,
                    4 => 3,
                    _ => self.direction
                };
                new_beam.push(Beam::new(self.pos, direction));
            },
            '\\' => {
                let direction = match self.direction {
                    1 => 4,
                    2 => 3,
                    3 => 2,
                    4 => 1,
                    _ => self.direction
                };
                new_beam.push(Beam::new(self.pos, direction));
            },
            '|' => {
                let direction = match self.direction {
                    2 =>{
                        new_beam.push(Beam::new(self.pos, 3));
                        1
                    },
                    4 => {
                        new_beam.push(Beam::new(self.pos, 3));
                        1
                    },
                    _ => self.direction
                };
                new_beam.push(Beam::new(self.pos, direction));
            },
            '-' => {
                let direction = match self.direction {
                    1 => {
                        new_beam.push(Beam::new(self.pos, 4));
                        2
                    },
                    3 => {
                        new_beam.push(Beam::new(self.pos, 4));
                        2
                    },
                    _ => self.direction
                };
                new_beam.push(Beam::new(self.pos, direction));
            },
            _ => new_beam.push(Beam::new(self.pos, self.direction)),
        }

        for beam in new_beam.iter_mut() {

            match beam.direction {
                1 => beam.pos.1 -= 1,
                2 => beam.pos.0 += 1,
                3 => beam.pos.1 += 1,
                4 => beam.pos.0 -= 1,
                _ => ()
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

#[derive(Debug , Clone, PartialEq, Eq)]
struct Box {
    tile_type: char,
    energized: (bool, Vec<u8>),
}



fn main() -> io::Result<()> {
    let file = File::open("/home/ersan/AOC2023/day16/input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<Vec<Box>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| Box {
                    tile_type: c,
                    energized: (false, Vec::new()),
                })
            .collect()
        })
    .collect();

    let mut max = 0;

    for y in 0..lines.len(){
        for x in 0..lines[0].len() {
            for i in 1..5 {
                let value = find_lenght(lines.clone(), (x as i32, y as i32), i);
                if value > max {
                    max = value;
                }
            } 
        }
        println!("{}: {}", y, max);
    }
    println!("{}", max);

    Ok(())
}




fn find_lenght(mut lines: Vec<Vec<Box>>, start_pos: (i32, i32), direction: i32) -> i32 {

    let mut beams: Vec<Beam> = vec![Beam::new((start_pos.0, start_pos.1), direction as u8)];

    while beams.len() > 0 {

        let mut beam = beams.pop().unwrap();
        let mut new_beams = beam.move_beam(&mut lines);
        beams.append(&mut new_beams);
    }

    lines.iter().flat_map(|line| line.iter()).filter(|box_| box_.energized.0).count() as i32


}