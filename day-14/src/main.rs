extern crate file_reader;
use file_reader::read_file;

use std::collections::HashMap;
use std::collections::VecDeque;

type Rules = HashMap<String, char>;

fn parse_rules(s: &str, rules: &mut Rules) {
    let t = s.split(" -> ").collect::<Vec<&str>>();

    rules.insert(t[0].to_string(), t[1].chars().collect::<Vec<char>>()[0]);
}

fn count_elements(template: &VecDeque<char>) -> HashMap<char, u32> {
    let mut counts: HashMap<char, u32> = HashMap::new();

    for c in template {
        if counts.contains_key(&c) {
            let mut val = *counts.get_mut(&c).unwrap() += 1;
        } else {
            counts.insert(c.clone(), 1);
        }
    }
    counts
}

fn process_template(rules: &Rules, template: &str, steps: usize) -> u32 {
    let mut stack_a: VecDeque<char> = VecDeque::new();

    for c in template.chars() {
        stack_a.push_back(c);
    }

    for r in 0..steps {
       //println!("STEP: {}", r);
        let mut stack_b: VecDeque<char> = VecDeque::new();
        while stack_a.len() > 0 {
            let mut pair = "".to_string();

            let a = stack_a.pop_front().unwrap();
            let b = stack_a.pop_front().unwrap();
            pair.push(a);
            pair.push(b);

            let c = rules.get(&pair).unwrap();
            stack_b.push_back(a);
            stack_b.push_back(c.clone());

            if stack_a.len() == 0 {
                stack_b.push_back(b)
            } else {
                stack_a.push_front(b);
            }
        }
        stack_a = stack_b;
        //println!("{}", stack_a.iter().collect::<String>());
    }

    let counts = count_elements(&stack_a);

    //println!("{:?}", counts);
    let mut vals: Vec<u32> = counts.values().cloned().collect();

    vals.sort();

    //println!("{:?}", vals);

    vals[vals.len() - 1] - vals[0]
}

fn part_1(rules: &Rules, template: &str) -> u32 {
    process_template(rules, template, 5)
}

#[derive(Debug)]
struct TwoPairs(String, String, char);


fn count3(counts: &mut HashMap<char, u64>, c: char, val: u64) {
    if counts.contains_key(&c) {
        *counts.get_mut(&c).unwrap() += val;
    } else {
        counts.insert(c, val);
    }
}

#[derive(Debug)]
struct TwoPairsA(String, String);

fn part_2(rules: &Rules, tmpl: &str) -> u64 {
    let steps = 40;
    let mut pair_counts: HashMap<String, u64> = HashMap::new();

    let keys = rules.keys();
    for k in keys {
        pair_counts.insert(k.clone(), 0);
    }

    let mut polymer = "".to_string();
    for c in tmpl.chars() {
        polymer.push(c);
        if polymer.len() == 2 {
            *pair_counts.get_mut(&polymer).unwrap() += 1;
            polymer = c.to_string();
        }
              
    }

    //println!("PAIR COUNTS : {:?}", pair_counts);

    for s in 0..steps {
        let step_count = pair_counts.clone();

        for (r, v) in rules {
            let pc = step_count.get(r).unwrap();

            let mut p_key_1 = "".to_string();
            let mut p_key_2 = "".to_string();
            let chars = r.chars().collect::<Vec<char>>();
            
            p_key_1.push(chars[0]);
            p_key_1.push(v.clone());

            p_key_2.push(v.clone());
            p_key_2.push(chars[1]);

            *pair_counts.get_mut(r).unwrap() -= pc;  

            *pair_counts.get_mut(&p_key_1).unwrap() += pc;
            
            *pair_counts.get_mut(&p_key_2).unwrap() += pc;
        }
    }

    let pc2s = pair_counts.clone();

        let mut counts: HashMap<char, u64> = HashMap::new();
        for (k, v) in pc2s {
            let chars = k.chars().collect::<Vec<char>>();
            count3(&mut counts, chars[1], v);
        }

    let mut vals: Vec<u64> = counts.values().cloned().collect();

    vals.sort();   

    vals[vals.len() - 1] - vals[0]

    
}

fn main() {
    let mut rules: Rules = Rules::new();
    let filename = "puzzle.txt";

    let mut template = "";
    if filename == "test01.txt" {
        template = "NNCB";
    } else {
        template = "OFSVVSFOCBNONHKFHNPK"; //"OFSVVSFOCBNONHKFHNPK"
    }

    read_file(filename, parse_rules, &mut rules);

    // println!("{:?}", rules);

    let ans_1 = part_1(&rules, &template);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2(&rules, &template);
    println!("Part 2: {}", ans_2);
}
