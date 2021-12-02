use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Command {
    Up(u32),
    Down(u32),
    Forward(u32),
   
}

type Commands = Vec<Command>;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(filename: &str) -> Commands {
    let mut commands: Commands = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let c  = ip.split(' ').collect::<Vec<&str>>();
                let val = c[1].parse::<u32>().unwrap();
                match c[0] {
                    "up" => commands.push(Command::Up(val)),
                    "down" => commands.push(Command::Down(val)),
                    "forward" => commands.push(Command::Forward(val)),
                    _ => panic!("invalid command")
                }
            }
        }
    }
    commands
}

fn part_1(commands: &Commands) -> u32 {
    let mut depth = 0;
    let mut hoz = 0;

    for command in commands {
        match command {
            Command::Up(n) => depth -= n,
            Command::Down(n) => depth += n,
            Command::Forward(n) => hoz += n
        }
    };

    depth * hoz    
}

fn part_2(commands: &Commands) -> u32 {
    let mut depth = 0;
    let mut hoz = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Command::Up(n) => aim -= n,
            Command::Down(n) => aim += n,
            Command::Forward(n) => {
                hoz += n;
                depth += aim * n
            }
        }
    };

    depth * hoz
}


fn main() {
    let commands = read_file("puzzle.txt");

    let ans_1 = part_1(&commands);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2(&commands);
    println!("Part 2: {}", ans_2);
}
