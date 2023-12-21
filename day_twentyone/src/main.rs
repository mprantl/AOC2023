use std::{fs::File, io::Read};

//function to read an input txt file into a Vec<Vec<char>>
fn read_file(filename: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();
    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    for c in contents.chars() {
        if c == '\n' {
            grid.push(row);
            row = Vec::new();
        } else {
            row.push(c);
        }
    }
    grid
}

//let S be the starting point. # are rocks, . are garden plots. the goal is to find the maximum nuber of reachable garden plots for a given number of steps.
// the player starts at the starting point S. they can move up, down, left, or right. they cannot move diagonally. they cannot move onto a rock. they can move onto a garden plot.
// find the maximum number of garden plots that can be reached in 6 steps.
fn find_reachable_plots(map: Vec<Vec<char>>, steps: i32) -> i32 {
    let mut reachable_plots = 0;
    let current_pos = (0, 0, steps); // Add steps to the current position
    let mut visited = Vec::new();
    let mut queue = Vec::new();
    queue.push(current_pos);
    while let Some((x, y, steps)) = queue.pop() {
        // Destructure the tuple here
        if steps < 0 {
            continue;
        }
        if !visited.contains(&(x, y)) {
            visited.push((x, y));
            if map[x][y] == '.' {
                reachable_plots += 1;
            }
            if x > 0 {
                queue.push((x - 1, y, steps - 1));
            }
            if x < map.len() - 1 {
                queue.push((x + 1, y, steps - 1));
            }
            if y > 0 {
                queue.push((x, y - 1, steps - 1));
            }
            if y < map[0].len() - 1 {
                queue.push((x, y + 1, steps - 1));
            }
        }
    }
    reachable_plots
}

fn main() {
    let map = read_file("input.txt");
    for row in &map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!("{}", find_reachable_plots(map, 64));
    //println!("{:?}", map);
}
