extern crate file_reader;
use file_reader::read_file;

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

fn parse_items(s: &str, items: &mut Vec<u32>) {
    let num = s.parse::<u32>().unwrap();
    items.push(num);
}

fn main() {
    let mut numbs: Vec<u32> = Vec::new();
    read_file("test01.txt", parse_items, &mut numbs);
    let p1_ans = part_1(&numbs);
    let p2_ans = part_2(&numbs);
    println!("Part 1 : {}", p1_ans);
    println!("Part 2 : {}", p2_ans);
}
