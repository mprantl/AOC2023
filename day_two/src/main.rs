use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

struct Round {
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

fn parse_lines(mut line: String) -> bool {
    let red_counter = 12;
    let green_counter = 13;
    let blue_counter = 14;

    line.retain(|c| !c.is_whitespace());
    let mut sep_line: Vec<_> = line.split(&[':', ';']).collect();
    sep_line.remove(0);

    for rounds in sep_line {
        let mut new_round = Round {
            red: 0,
            green: 0,
            blue: 0,
        };
        for results in rounds.split(",") {
            if results.contains("red") {
                let number = results.replace("red", "");
                new_round.red += number.parse::<i32>().unwrap();
                if new_round.red > red_counter {
                    return false;
                }
            }
            if results.contains("green") {
                let number = results.replace("green", "");
                new_round.green += number.parse::<i32>().unwrap();
                if new_round.green > green_counter {
                    return false;
                }
            }
            if results.contains("blue") {
                let number = results.replace("blue", "");
                new_round.blue += number.parse::<i32>().unwrap();
                if new_round.blue > blue_counter {
                    return false;
                }
            }
        }
    }
    true
}

fn parse_lines_v2(mut line: String) -> Round {
    let mut new_round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };
    line.retain(|c| !c.is_whitespace());
    let mut sep_line: Vec<_> = line.split(&[':', ',', ';']).collect();
    sep_line.remove(0);
    for rounds in sep_line {
        for results in rounds.split(',') {
            if rounds.contains("red") {
                let number = results.replace("red", "").parse::<i32>().unwrap();
                if number > new_round.red {
                    new_round.red = number;
                }
            }
            if rounds.contains("green") {
                let number = results.replace("green", "").parse::<i32>().unwrap();
                if number > new_round.green {
                    new_round.green = number;
                }
            }
            if rounds.contains("blue") {
                let number = results.replace("blue", "").parse::<i32>().unwrap();
                if number > new_round.blue {
                    new_round.blue = number;
                }
            }
        }
    }
    new_round
}

fn main() {
    let filename = "input.txt";
    let mut sum = 0;
    let mut game_counter = 1;
    let mut power = 0;
    let mut power_sum = 0;

    match read_lines(filename) {
        Ok(lines) => {
            for line in lines {
                let game = parse_lines_v2(line.unwrap());
                power = game.red * game.green * game.blue;
                println!(
                    "------------------------- GAME {} --------------------------",
                    game_counter
                );
                println!(
                    "Summary: red: {} \t green: {} \t blue: {} \t power: {}",
                    game.red, game.green, game.blue, power
                );
                println!("-----------------------------------------------------------");

                power_sum += power;
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    }
    println!("POWERSUM: {}", power_sum);

    match read_lines(filename) {
        Ok(lines) => {
            for line in lines {
                if parse_lines(line.unwrap()) == true {
                    sum += game_counter;
                }
                game_counter += 1;
                /*
                println!(
                    "------------------------- GAME {} --------------------------",
                    game_counter
                );
                println!(
                    "Summary: red: {} \t green: {} \t blue: {} \t",
                    game.red, game.green, game.blue
                );
                if game.red <= red_counter
                    && game.green <= green_counter
                    && game.blue <= blue_counter
                {
                    print!("VALID! \t\t");
                    sum += game_counter;
                    println!("Current sum: {}", sum);
                    println!("-----------------------------------------------------------");
                } else {
                    print!("INVALID! \t");
                    println!("Current sum: {}", sum);
                    println!("-----------------------------------------------------------");
                }*/
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    }
    println!("FINAL: {sum}");
}
