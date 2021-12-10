extern crate file_reader;
use file_reader::read_file;

use std::collections::VecDeque;

type Chunks = Vec<Vec<char>>;
type ParsedChunks = Vec<VecDeque<char>>;

fn parse_chunks(s: &str, chunks: &mut Chunks) {
    let line = s.chars().collect::<Vec<char>>();
    chunks.push(line)
}

fn part_1(chunks: &Chunks) -> u64 {
    let mut stack: VecDeque<char> = VecDeque::new();
    let mut score = 0;

    for line in chunks {
        stack = VecDeque::new();

        for c in line {
            match c {
                '(' => stack.push_front('('),
                '[' => stack.push_front('['),
                '{' => stack.push_front('{'),
                '<' => stack.push_front('<'),
                ')' => {
                    if &'(' != &stack.pop_front().unwrap() {
                        score += 3;
                        break;
                    }
                }
                ']' => {
                    if &'[' != &stack.pop_front().unwrap() {
                        score += 57;
                        break;
                    }
                }
                '}' => {
                    if &'{' != &stack.pop_front().unwrap() {
                        score += 1197;
                        break;
                    }
                }
                '>' => {
                    if &'<' != &stack.pop_front().unwrap() {
                        score += 25137;
                        break;
                    }
                }
                _ => panic!("{}", "Invalid character!"),
            }
        }
    }

    score
}

fn purge_corrupted_lines(chunks: &Chunks) -> ParsedChunks {
    let mut parsed: ParsedChunks = ParsedChunks::new();
    for line in chunks {
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut corrupt = false;
        for c in line {
            match c {
                '(' => stack.push_front('('),
                '[' => stack.push_front('['),
                '{' => stack.push_front('{'),
                '<' => stack.push_front('<'),
                ')' => {
                    if &'(' != &stack.pop_front().unwrap() {
                        corrupt = true;
                        break;
                    }
                }
                ']' => {
                    if &'[' != &stack.pop_front().unwrap() {
                        corrupt = true;
                        break;
                    }
                }
                '}' => {
                    if &'{' != &stack.pop_front().unwrap() {
                        corrupt = true;
                        break;
                    }
                }
                '>' => {
                    if &'<' != &stack.pop_front().unwrap() {
                        corrupt = true;
                        break;
                    }
                }
                _ => panic!("{}", "Invalid character!"),
            }
        }

        if !corrupt {
            parsed.push(stack)
        }
    }

    parsed
}

fn part_2(chunks: &Chunks) -> u64 {
    let mut parsed = purge_corrupted_lines(chunks);
    let mut scores: Vec<u64> = Vec::new();

    for n in 0..parsed.len() {
        let mut score = 0;
        while parsed[n].len() > 0 {
            let c = parsed[n].pop_front().unwrap();
            match c {
                '(' => score = score * 5 + 1,
                '[' => score = score * 5 + 2,
                '{' => score = score * 5 + 3,
                '<' => score = score * 5 + 4,
                _ => panic!("{}", "Invalid char found in stack!"),
            }
        }
        scores.push(score);
    }

    scores.sort();

    //println!("SCORES::::: {:?}", scores);
    scores[scores.len() / 2]
}

fn main() {
    let mut chunks: Chunks = Chunks::new();
    read_file("puzzle.txt", parse_chunks, &mut chunks);

    let ans_1 = part_1(&chunks);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2(&chunks);
    println!("Part 2: {}", ans_2);
}
