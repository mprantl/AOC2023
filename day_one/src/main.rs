use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_numbers(line: String) -> i32{
    let chars: Vec<char> = line.chars().collect();
    //println!("char: {:?}", chars);
    let mut result:Vec<char>= vec![];
    for i in chars{
        if i.is_numeric(){
            result.push(i);
        }
    }
    let first = result[0];
    let last = result[result.len()-1];
    let fandl = vec![first, last];
    let s: String = fandl.into_iter().collect();
    let real_result =  s.parse::<i32>().unwrap();
    real_result
}

fn main() {
    let filename = "input.txt";
    let mut sum = 0;

    match read_lines(filename) {
        Ok(lines) => {
            for line in lines {
               let x = find_numbers(line.unwrap());
               println!("{}", x);
               sum += x;
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    }
    println!("{sum}");
}
