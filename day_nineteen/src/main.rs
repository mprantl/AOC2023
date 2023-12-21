use std::collections::HashMap;
use std::io::{self, BufRead};

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

struct Rule {
    condition: String,
    workflow: String,
}

struct Workflow {
    rules: Vec<Rule>,
}

fn process_part(part: &Part, workflows: &HashMap<String, Workflow>) -> Option<i32> {
    let mut current_workflow = workflows.get("in").unwrap();

    loop {
        for rule in &current_workflow.rules {
            let parts: Vec<&str> = rule.condition.split(|c| c == '<' || c == '>').collect();
            let value = match parts[0] {
                "x" => part.x,
                "m" => part.m,
                "a" => part.a,
                "s" => part.s,
                _ => continue,
            };
            let condition_value: i32 = parts[1].parse().unwrap();
            let condition_met = if rule.condition.contains('>') {
                value > condition_value
            } else {
                value < condition_value
            };

            if condition_met {
                if rule.workflow == "A" {
                    return Some(part.x + part.m + part.a + part.s);
                } else if rule.workflow == "R" {
                    return None;
                } else {
                    current_workflow = workflows.get(&rule.workflow).unwrap();
                    break;
                }
            }
        }
    }
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_input(
    filename: &str,
) -> io::Result<(
    HashMap<String, HashMap<String, String>>,
    Vec<HashMap<String, i32>>,
)> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    let mut is_part = false;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('{') {
            is_part = true;
        }

        if is_part {
            let mut part = HashMap::new();
            for item in line.trim_matches(|c| c == '{' || c == '}').split(',') {
                let kv: Vec<&str> = item.split('=').collect();
                part.insert(kv[0].to_string(), kv[1].parse().unwrap());
            }
            parts.push(part);
        } else {
            let mut workflow = HashMap::new();
            let items: Vec<&str> = line.split('{').collect();
            let name = items[0].trim().to_string();
            for rule in items[1].trim_matches('}').split(',') {
                let kv: Vec<&str> = rule.split(':').collect();
                workflow.insert(kv[0].to_string(), kv[1].to_string());
            }
            workflows.insert(name, workflow);
        }
    }

    Ok((workflows, parts))
}

fn main() {
    let filename = "input-smoll.txt";
    match parse_input(filename) {
        Ok((workflows, parts)) => {
            let mut total = 0;
            for part in parts {
                if let Some(rating) = process_part(&part, &workflows) {
                    total += rating;
                }
            }

            println!("{}", total);
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}
