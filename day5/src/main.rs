use std::fs::File;
use std::io::{self, BufRead, BufReader};





fn main() -> io::Result<()> {


    let file = File::open("/home/ersan/AOC2023/day5/input.txt")?;
    let reader = BufReader::new(file);
    
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    
    
    
    let seeds: Vec<i64> = lines[0][6..]
    .split(' ')
    .filter_map(|s| s.parse().ok())
    .collect();


/*  PART 1
    let mut lowest: i64 = 1000000000000;

    for seed in seeds {
        
        let value = big_mapper(seed, &lines);
        if value < lowest {
            lowest = value;
        }
    }
    println!("{}", lowest);
*/

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    for chunk in seeds.chunks(2) {
        ranges.push((chunk[0], chunk[1]));
    }

    for range in ranges.iter() {
        println!("{:?}", range);
    }
    
    
    
    
    
    Ok(())
}

fn big_mapper(seed: i64, lines: &Vec<String>) -> i64 {

    let mut value = seed;
    // seed to soil
    value = mapper(value, lines[3..10].to_vec());
    // soil to fertilizer
    value = mapper(value, lines[12..58].to_vec());
    // fertilizer to water
    value = mapper(value, lines[60..109].to_vec());
    // water to light
    value = mapper(value, lines[111..134].to_vec());
    // light to temprature
    value = mapper(value, lines[136..182].to_vec());
    // temprature to humidity
    value = mapper(value, lines[184..200].to_vec());
    // humidity to location
    value = mapper(value, lines[202..226].to_vec());
        
    
    return value;

}


fn mapper(value: i64, map: Vec<String>) -> i64 {
    for x in map.iter() {
        let range: Vec<i64> = x
        .split(' ')
        .filter_map(|s| s.parse().ok())
            .collect();

        let source = range[1]; 
        let destination = range[0];

        if value >= source {
            if value <= source + range[2] - 1 {
                return (value - source) + destination;    
            }
        }
    }
    return value;
}
