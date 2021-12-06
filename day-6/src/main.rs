use std::collections::HashMap;
use std::fs;

type SchoolOfFish = Vec<i32>;

fn read_spawn_days(filename: &str) -> SchoolOfFish {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    contents
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn advance_day(school: &mut SchoolOfFish) {
    let mut new_fish: SchoolOfFish = SchoolOfFish::new();
    for f in 0..school.len() {
        school[f] -= 1;
        if school[f] < 0 {
            school[f] = 6;
            new_fish.push(8)
        }
        if new_fish.len() > 0 {
            school.append(&mut new_fish);
            new_fish = SchoolOfFish::new();
        }
    }

    school.append(&mut new_fish);
}

fn part_1(school: &mut SchoolOfFish, days: u32) {
    for _ in 0..days {
        advance_day(school);
    }
}

fn how_may_fish(s: i32, days: i32, f_cache: &mut HashMap<i32, u64>) -> u64 {
    let mut total: u64 = 0;
    let first_spawn_day = days - (s + 1);
    if first_spawn_day < 0 {
        return 0;
    }
    let mut new_fish = (first_spawn_day / 7 + 1) as i64;

    total += new_fish as u64;
    let mut next_spawn_day: i32 = first_spawn_day;

    while new_fish > 0 {
        if f_cache.contains_key(&next_spawn_day) {
            total += f_cache.get(&next_spawn_day).unwrap();
        } else {
            let e = how_may_fish(8, next_spawn_day, f_cache);
            total += e;
            f_cache.insert(next_spawn_day, e);
        }
        new_fish -= 1;

        next_spawn_day = next_spawn_day - 7;
    }
    total
}

fn part_2(school: SchoolOfFish, days: i32) -> u64 {
    let mut count = school.len() as u64;
    let mut cache: HashMap<i32, u64> = HashMap::new();
    let mut f_cache: HashMap<i32, u64> = HashMap::new();

    for s in school {
        if cache.contains_key(&s) {
            count += cache.get(&s).unwrap();
            continue;
        }
        let new_fish = how_may_fish(s, days, &mut f_cache);
        cache.insert(s, new_fish);
        count += new_fish;
    }
    count
}

fn main() {
    let mut school = read_spawn_days("puzzle.txt");

    let s2 = school.clone();

    part_1(&mut school, 80);
    println!("Part 1: {}", school.len());

    let ans_2 = part_2(s2, 256);
    println!("Part 2: {}", ans_2);
}
