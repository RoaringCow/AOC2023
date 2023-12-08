use std::fs::File;
use std::io::{self, BufRead, BufReader};



fn main() -> io::Result<()> {
    
    
    let file = File::open("/home/ersan/AOC2023/day7/input.txt")?;
    let reader = BufReader::new(file);
    
    
    let mut hands: Vec<(String, u32)> = Vec::new();
    
    // what the fuck is this
    let mut five_of_akind: Vec<(String, u32)> = Vec::new();
    let mut four_of_akind: Vec<(String, u32)> = Vec::new();
    let mut full_house: Vec<(String, u32)> = Vec::new();
    let mut three_of_akind: Vec<(String, u32)> = Vec::new();
    let mut two_pair: Vec<(String, u32)> = Vec::new();
    let mut one_pair: Vec<(String, u32)> = Vec::new();
    let mut high_card: Vec<(String, u32)> = Vec::new();
    
    for line_result in reader.lines() {
        let line = line_result?;
        
        let mut hand_map: std::collections::BTreeMap<char, u32> = std::collections::BTreeMap::new();
        
        let split = line.split(" ").collect::<Vec<&str>>();
        
        let bid = split[1].parse::<u32>().unwrap();
        
        let mut card = split[0].replace("T", &char::from_u32('9' as u32 + 1).unwrap().to_string())
                                    .replace("J", &char::from_u32('2' as u32 - 1).unwrap().to_string())
                                    .replace("Q", &char::from_u32('9' as u32 + 3).unwrap().to_string())
                                    .replace("K", &char::from_u32('9' as u32 + 4).unwrap().to_string())
                                    .replace("A", &char::from_u32('9' as u32 + 5).unwrap().to_string());

        for c in card.to_lowercase().chars() {
            *hand_map.entry(c).or_insert(0) += 1;
        }
        
        let mut sorted: Vec<(char, u32)> = Vec::new();

        for x in hand_map.iter() {
            let (character, value) = x;
            sorted.push((character.to_owned(), value.to_owned()))
        }
        
        sorted.sort_by_key(|a| a.1);
        
        let mut len = sorted.len();
        
        if len != 1 && sorted.last().unwrap().0 != '1' {
            
            for (i, element) in sorted.clone().iter_mut().enumerate() {
                if element.0 == '1' {
                    let joker_count = element.1;
                    sorted.remove(i);
                    len -= 1;
                    sorted.last_mut().unwrap().1 += joker_count;
                }
            }
        }
        
        for x in sorted.iter(){
            //println!("{:?}  {}  {}", x, card, len);
        }
    
    
        match len {
            1 => five_of_akind.push((card.to_string(), bid)),
            2 => {
                if sorted[1].1 == 4 {
                    four_of_akind.push((card.to_string(), bid))
                }else {
                    full_house.push((card.to_string(), bid))
                }
            },
            3 => {
                if sorted[2].1 == 3 {
                    three_of_akind.push((card.to_string(), bid))
                } else {
                    two_pair.push((card.to_string(), bid))
                }
            },
            4 => one_pair.push((card.to_string(), bid)),
            _ => high_card.push((card.to_string(), bid))
            
        };
        //println!();

    }
    
    
    // what the fuck is this

    five_of_akind.sort_by(|a, b| a.0.cmp(&b.0));
    four_of_akind.sort_by(|a, b| a.0.cmp(&b.0));
    full_house.sort_by(|a, b| a.0.cmp(&b.0));
    three_of_akind.sort_by(|a, b| a.0.cmp(&b.0));
    two_pair.sort_by(|a, b| a.0.cmp(&b.0));
    one_pair.sort_by(|a, b| a.0.cmp(&b.0));
    high_card.sort_by(|a, b| a.0.cmp(&b.0));
    

    hands.extend(high_card);
    hands.extend(one_pair);
    hands.extend(two_pair);
    hands.extend(three_of_akind);
    hands.extend(full_house);
    hands.extend(four_of_akind);
    hands.extend(five_of_akind);

    let mut sum = 0;

    for (i, card) in hands.iter().enumerate() {

        sum += card.1 * (i + 1) as u32;
        println!("{}  : {}", card.0, card.1);
    }
    println!("{}", sum);
    Ok(())
}
