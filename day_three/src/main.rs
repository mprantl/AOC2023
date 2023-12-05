use std::error::Error;
use std::fs::read_to_string;

struct Number{
    start_index: 
}

fn read_file_to_2d_array(filename: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let content = read_to_string(filename)?;
    let mut array_2d = Vec::new();

    for line in content.lines() {
        let row = line.chars().map(|c| c.to_string()).collect::<Vec<String>>();
        array_2d.push(row);
    }

    Ok(array_2d)
}

fn parse_array(array: Vec<Vec<String>>) {
    for row in &array {
        for elem in row {
            print!("{}\t", elem);
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let array_2d = read_file_to_2d_array("input-smoll.txt")?;
    parse_array(array_2d);
    Ok(())
}
