use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(filename: &str) -> Vec<u32> {
    let mut numbs = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let num = ip.parse::<u32>().unwrap();
                numbs.push(num);
            }
        }
    }
    numbs
}

fn part_1(numbs: &Vec<u32>) -> u32 {
    let mut increases = 0;

    for i in 1..numbs.len() {
        if numbs[i] > numbs[i - 1] {
            increases += 1;
        }
    }
    increases
}

fn part_2(numbs: &Vec<u32>) -> u32 {
    let mut increases = 0;

    let mut window = numbs[0] + numbs[1] + numbs[2];
    for i in 3..numbs.len() {
        let tmp = window;
        window = (window - numbs[i - 3]) + numbs[i];
        if window > tmp {
            increases += 1;
        }
    }
    increases
}

fn main() {
    let numbs = read_file("puzzle.txt");
    let p1_ans = part_1(&numbs);
    let p2_ans = part_2(&numbs);
    println!("Part 1 : {}", p1_ans);
    println!("Part 2 : {}", p2_ans);
    println!("Hello, world!");
}
