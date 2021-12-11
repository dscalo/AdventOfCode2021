use std::fs;

fn read_position(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    contents
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn get_max_value(numbs: &Vec<i32>) -> i32 {
    let mut max = i32::MIN;

    for n in 0..numbs.len() {
        if numbs[n] > max {
            max = numbs[n];
        }
    }
    max
}

fn part_1(position: &Vec<i32>) -> i32 {
    let mut lowest_cost = i32::MAX;
    let max = get_max_value(&position);

    for  mv in 1..=max  {
        let mut tmp = 0;
        for p in 0..position.len() {           
            tmp += (position[p] - mv).abs();
        }
        

        if tmp < lowest_cost {
            lowest_cost = tmp
        }
    }

    lowest_cost
}

fn gauss(n: i32) -> i32 {
   (n * (n + 1)) / 2
}

fn part_2(position: &Vec<i32>) -> i32 {
    let mut lowest_cost = i32::MAX;
    let max = get_max_value(&position);

    for  mv in 1..=max  {
        let mut tmp = 0;
        for p in 0..position.len() {           
            tmp += gauss((position[p] - mv).abs());
        }
        

        if tmp < lowest_cost {
            lowest_cost = tmp
        }
    }
    lowest_cost   
}

fn main() {
    let position = read_position("puzzle.txt");

    let ans_1 = part_1(&position);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2(&position);
    println!("Part 2: {}", ans_2);
}
