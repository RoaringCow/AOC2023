use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;


#[allow(dead_code)]
#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    last_option: String
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rule {
    value: i32,
    categorie: u8,
    // < is true, > is false
    comparison_type: bool,
    destination: String
}



#[allow(dead_code, unused_variables,)]
fn main() -> io::Result<()> {
    let file = File::open("/home/ersan/AOC2023/day19/input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    for line in lines[..572].iter() {
        let mut workflow = Workflow {
            name: String::new(),
            rules: Vec::new(),
            last_option: String::new()
        };
        let splitted_line = line.split('{').collect::<Vec<&str>>();
        let rules = &splitted_line[1][..splitted_line[1].len() - 1].split(',').collect::<Vec<&str>>();
        workflow.name = splitted_line[0].to_string();
        workflow.last_option = rules[rules.len() - 1].to_string();
        for rule in rules[..rules.len() - 1].iter() {
            let split = rule.split(':').collect::<Vec<&str>>();
            let comparison_type = split[0].contains("<");
            let value = split[0][2..].parse::<i32>().unwrap();
            let destination = split[1].to_string();
            let categorie = match split[0].chars().nth(0).unwrap() {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => 230
            };
            workflow.rules.push(Rule {
                value,
                comparison_type,
                destination,
                categorie,
            });
        }
        workflows.insert(workflow.name.clone(), workflow);
    }

    let parts: Vec<Vec<i32>> = lines[573..].iter().map(|l| l[1..l.len() - 1].split(',').map(|n| n[2..].to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect(); 


    let mut sum = 0;

    for part in parts.iter() {
        //println!("Part: {:?}", part);
        let mut current_workflow = String::from("in");
        while &current_workflow != "A" && &current_workflow != "R" {
            let mut does_match = false;
            for (i, rule) in workflows.get(&current_workflow).unwrap().rules.iter().enumerate() {
                if does_match {
                    break;
                }
                //println!("Rule: {:?}    {}", rule, part[rule.categorie as usize]);
                match rule.comparison_type {
                    true => 
                        if part[rule.categorie as usize] < rule.value {
                            current_workflow = rule.destination.clone();
                            does_match = true;
                        }
                    false =>
                        if part[rule.categorie as usize] > rule.value {
                            current_workflow = rule.destination.clone();
                            does_match = true;
                        }
                }
            }

            if !does_match {
                current_workflow = workflows.get(&current_workflow).unwrap().last_option.clone();
            }

        }
        let mut rule_sum = 0;
        if current_workflow == "A" {
            rule_sum = part.iter().sum();
        }
        sum += rule_sum;

        
    }

    println!("Sum: {}", sum);
    Ok(())
}
