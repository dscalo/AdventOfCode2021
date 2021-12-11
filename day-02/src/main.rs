extern crate file_reader;
use file_reader::read_file;

#[derive(Debug)]
enum Command {
    Up(u32),
    Down(u32),
    Forward(u32),
}

type Commands = Vec<Command>;

fn parse_commands(s: &str, commands: &mut Commands) {
    let c = s.split(' ').collect::<Vec<&str>>();
    let val = c[1].parse::<u32>().unwrap();
    match c[0] {
        "up" => commands.push(Command::Up(val)),
        "down" => commands.push(Command::Down(val)),
        "forward" => commands.push(Command::Forward(val)),
        _ => panic!("invalid command"),
    }
}

fn part_1(commands: &Commands) -> u32 {
    let mut depth = 0;
    let mut hoz = 0;

    for command in commands {
        match command {
            Command::Up(n) => depth -= n,
            Command::Down(n) => depth += n,
            Command::Forward(n) => hoz += n,
        }
    }

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
    }

    depth * hoz
}

fn main() {
    let mut commands = Commands::new();
    read_file("puzzle.txt", parse_commands, &mut commands);

    let ans_1 = part_1(&commands);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2(&commands);
    println!("Part 2: {}", ans_2);
}
