use std::collections::HashMap;
extern crate file_reader;
use file_reader::read_file;

struct Signal {
    pattern: Vec<String>,
    output: Vec<String>
}

impl Signal {
    fn new(p: &str, o: &str) -> Signal {        
        Signal {
            pattern: p.split_whitespace().map(str::to_string).collect::<Vec<String>>(),
            output: o.split_whitespace().map(str::to_string).collect::<Vec<String>>()
        }
    }
    fn pretty_print(&self) {
        for p in &self.pattern {
            print!("{} ", p)
        }

        print!("| ");

        for o in &self.output {
            print!("{} ", o);
        }
        println!(" ");
    }
}

type Signals = Vec<Signal>;


fn parse_signals(s: &str, signals: &mut Signals) { 
    // 2,2 -> 2,1
    let segments = s.split(" | ").collect::<Vec<&str>>();

    let signal = Signal::new(segments[0], segments[1]);

    signals.push(signal);
}

fn part_1(signals: &Signals) -> u32 {
    let mut count = 0;

    for signal in signals {
        for o in &signal.output {
            match o.len() {
                2 => count += 1,
                3 => count += 1,
                4 => count += 1,
                7 => count += 1,
                _ => {}
                
            }
        }
    }

    count
}
/*
    111111
    2    3
    2    3
    444444
    5    6
    5    6
    777777
*/

fn find_segments(patterns: &Vec<String>, segments: &mut HashMap<char, char>) {
    let mut one = "".to_string();
    let mut three = "".to_string();
    let mut four = "".to_string();
    let mut seven = "".to_string();
    let mut two_3_5: Vec<String> = Vec::new();


    for pattern in patterns {

        match pattern.len() {
            2 => {one = pattern.clone()},
            3 => {seven = pattern.clone()}, 
            4 => {four = pattern.clone()},
            5 => {
                    let mut cha = pattern.chars().collect::<Vec<char>>();

                    cha.sort();

                    let p: String = cha.into_iter().collect();

                    if !two_3_5.contains(&p) {
                        two_3_5.push(p);
                    }
            },
            _ => {}
        }
    }
    
    let one_chars = one.chars().collect::<Vec<char>>();
    let four_chars = four.chars().collect::<Vec<char>>();
    let seven_chars = seven.chars().collect::<Vec<char>>();

    // get top
    for c in &seven_chars {
        if !one_chars.contains(&c) {
            segments.insert(c.clone(), '1');
            break;
        }
    }

    // find three      
    for n in &two_3_5 {
        let c = n.chars().collect::<Vec<char>>();
        if c.contains(&one_chars[0]) && c.contains(&one_chars[1]) {
            three = n.clone();    
            continue;       
        }
    }
    let three_chars = three.chars().collect::<Vec<char>>();

    // get middle and bottom segments
    for c in &three_chars {
        if seven_chars.contains(&c) {
            // top or right segment
            continue;
        }

        if four_chars.contains(&c) {
            // middle
            segments.insert(c.clone(), '4');
            continue;
        }

        // bottom
        segments.insert(c.clone(), '7');
    }


    for c in &four_chars {
        if !three_chars.contains(&c) {
            // top left
            segments.insert(c.clone(), '2');
            break
        }
    }

   
    // find  2 and 5
    let mut two = "".to_string();
    let mut five = "".to_string();
    for n in &two_3_5 {
        if n == &three {
            continue
        }
        let mut tr = '-';

        for (k, v) in &*segments {
            if v == &'2' {
                tr = k.clone();
            }
        }

        if n.chars().collect::<Vec<char>>().contains(&tr) {
            // 5
            five = n.clone();
        } else {
            two = n.clone();
        }
    }
    let two_chars = two.chars().collect::<Vec<char>>();
    let five_chars = five.chars().collect::<Vec<char>>();

    // find right segments 
    if five_chars.contains(&one_chars[0]) &&!two_chars.contains(&one_chars[0]) {
        // top right - bottom right
        segments.insert(one_chars[0].clone(),'6');
        segments.insert(one_chars[1].clone(),'3');
    } else {
        segments.insert(one_chars[0].clone(),'3');
        segments.insert(one_chars[1].clone(), '6');
    }

    // find bottom left
    for c in &two_chars {
        if !segments.contains_key(&c) {
            segments.insert(c.clone(), '5');
           break;
        }      
    }
}

fn lookup(s: &str) -> String {

    let mut sorted_s = s.chars().collect::<Vec<char>>();
    sorted_s.sort();

    let st = sorted_s.into_iter().collect::<String>();

    let zero = "123567";
    let two = "13457";
    let three = "13467";
    let five = "12467";
    let six = "124567";
    let nine = "123467";    

    if st == zero {
        return "0".to_string()
    }

    if st == two {
        return "2".to_string()
    }

    if st == three {
        return "3".to_string()
    }

    if st == five {
        return "5".to_string()
    }

    if st == six {
        return "6".to_string()
    }

    if st == nine {
        return "9".to_string()
    }

    panic!("{}", "Invalid string")
}

fn decode(patterns: &Vec<String>) -> HashMap<String, String> {
    let mut decoder: HashMap<String, String> = HashMap::new();
    let mut segments: HashMap<char, char> = HashMap::new();

    find_segments(&patterns, &mut segments);

    for pattern in patterns {
        if decoder.contains_key(pattern) {
            continue;
        }
        match pattern.len() {
            2 => {
                decoder.insert(pattern.clone(), "1".to_string());            
            },
            3 => {
                decoder.insert(pattern.clone(), "7".to_string());    
            },
            4 => {
                decoder.insert(pattern.clone(), "4".to_string());   
            },
            7 => {
                decoder.insert(pattern.clone(), "8".to_string());   
            },

            _ => {
                let chars = pattern.chars().collect::<Vec<char>>();
                let mut segs = "".to_string();

                for c in chars {
                    segs.push(segments.get(&c).unwrap().clone());
                }
                decoder.insert(pattern.clone(), lookup(&segs));
            }
        }
    }

    decoder

}

fn part_2(signals: &Signals) -> u32 {
    let mut sum = 0;
    
    for signal in signals {
        let mut p: Vec<String> = signal.pattern.clone();
        p.append(&mut signal.output.clone());
        let decoder = decode(&p);
       
        let mut str_val   = "".to_string();

        for output in &signal.output {
            let val = decoder.get(output).unwrap();
            str_val.push_str(val);
        }

        let val = str_val.parse::<u32>().unwrap();

        sum += val;
    }

    sum
}


fn main() {
    let mut signals: Signals = Signals::new();
    read_file("puzzle.txt", parse_signals, &mut signals);

    let ans_1 = part_1(&signals);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2(&signals);
    println!("Part 2: {}", ans_2);
}
