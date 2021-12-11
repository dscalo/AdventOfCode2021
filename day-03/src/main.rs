extern crate file_reader;
use file_reader::read_file;

type Report = Vec<Vec<char>>;

#[derive(Debug)]
struct BitsCount(u32, u32);

fn parse_bits(s: &str, report: &mut Report) {  
    report.push(s.chars().collect::<Vec<char>>());
}

fn part_1(report: &Report) -> u32 {
    let mut pos: Vec<BitsCount> = Vec::new();
    for _ in 0..report[0].len() {
        pos.push(BitsCount(0, 0));
    }

    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    for row in report {
        for b in 0..row.len() {
            match row[b] {
                '0' => pos[b].0 += 1,
                '1' => pos[b].1 += 1,
                _ => panic!("{}", "invalid char"),
            };
        }
    }

    for p in pos {
        if p.0 > p.1 {
            gamma.push_str("0");
            epsilon.push_str("1");
        } else {
            gamma.push_str("1");
            epsilon.push_str("0");
        }
    }

    println!("Gamma: {}, epsolin: {}", gamma, epsilon);

    let g = u32::from_str_radix(&gamma, 2).unwrap();
    let e = u32::from_str_radix(&epsilon, 2).unwrap();

    g * e
}

fn bit_freq(p: usize, report: &Report) -> BitsCount {
    let mut pos = BitsCount(0, 0);
    for row in report {
        match row[p] {
            '0' => pos.0 += 1,
            '1' => pos.1 += 1,
            _ => panic!("{}", "invalid number"),
        }
    }
    pos
}

fn filter_report(p: usize, keep: char, report: &Report) -> Report {
    let mut filtered = Report::new();

    for row in report {
        if row[p] == keep {
            filtered.push(row.clone());
        }
    }
    filtered
}

fn part_2(report: &Report) -> u32 {
    let row_size = report[0].len();
    let mut oxygen = report.clone();
    let mut c02 = report.clone();

    while oxygen.len() > 1 {
        for p in 0..row_size {
            let ct = bit_freq(p, &oxygen);
            let mut keep = '1';
            if ct.0 > ct.1 {
                keep = '0';
            }
            oxygen = filter_report(p, keep, &oxygen);
            if oxygen.len() == 1 {
                break;
            }
        }
    }

    while c02.len() > 1 {
        for p in 0..row_size {
            let ct = bit_freq(p, &c02);
            let mut keep = '0';
            if ct.1 < ct.0 {
                keep = '1';
            }
            c02 = filter_report(p, keep, &c02);
            if c02.len() == 1 {
                break;
            }
        }
    }
    
    let o_str = oxygen[0].iter().collect::<String>();
    let c_str = c02[0].iter().collect::<String>();

    println!("o: {}, c: {}", o_str, c_str);
    let o = u32::from_str_radix(&o_str, 2).unwrap();
    let c = u32::from_str_radix(&c_str, 2).unwrap();

    o * c
}

fn main() {
    let mut report = Report::new();
    read_file("puzzle.txt", parse_bits, &mut report);

    let ans_1 = part_1(&report);
    let ans_2 = part_2(&report);
    println!("Part 1: {}", ans_1);
    println!("Part 2: {}", ans_2);    
}
