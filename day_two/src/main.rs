use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::path::Path;

struct Game{
    index: i32,
    red: i32,
    green: i32,
    blue: i32,
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_lines(mut line: String){
    
    let mut sep_line: Vec<_> = vec![];
    //sep_line = line.split_whitespace().collect();let mut blue_counter = 14;
    //let v: Vec<&str> = "2020-11-03 23:59".split(&['-', ' ', ':', '@'][..]).collect();
    line.retain(|c| !c.is_whitespace());
    sep_line = line.split(&[':',';']).collect();
    println!("{:?}", sep_line);

}

fn main() {
    let filename = "input-smoll.txt";
    let mut sum = 0;

    let mut red_counter = 12;
    let mut green_counter = 13;
    let mut blue_counter = 14;

    match read_lines(filename) {
        Ok(lines) => {
            for line in lines {
              //println!("{}", line.unwrap());
              parse_lines(line.unwrap());
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    }
    println!("{sum}");
}