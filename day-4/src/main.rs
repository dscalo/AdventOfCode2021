use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod bingo;

use crate::bingo::BingoBoard;

type Draws = Vec<u32>;

type Boards = Vec<BingoBoard>;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file(filename: &str) -> Boards {
    let mut boards: Boards = Boards::new();
    let mut board_numbs: Vec<Vec<u32>> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() == 0 {
                    let b = BingoBoard::new(board_numbs);
                    boards.push(b);
                    board_numbs = Vec::new();
                    continue;
                }
                let row = ip.split(" ");

                let mut tmp: Vec<u32> = Vec::new();
                for r in row {
                    if r.len() == 0 {
                        continue;
                    }
                    let n = r.parse::<u32>().unwrap();
                    //println!("pushing in {}", n);
                    tmp.push(n);
                    
                }

                board_numbs.push(tmp);
            }
        }
    }

    if board_numbs.len() > 0 {
        let b = BingoBoard::new(board_numbs);
        boards.push(b);
    }

    boards
}

fn part_1_and_2(boards: &mut Boards, draws: &Draws) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut skip: Vec<usize> = Vec::new();

    for draw in draws {
        for i in 0..boards.len() {
             if skip.contains(&i) {
                 continue;
             }

            boards[i].mark(*draw);
            if boards[i].bingo() {
               if part1 == 0 {
                part1 = boards[i].unmarked_sum() * draw
               }

               skip.push(i);

               if skip.len() == boards.len() {
                   part2 =  boards[i].unmarked_sum() * draw;
                   break;
               }
            
            }
        }
    }

    (part1, part2)
}


fn read_draw_numbers(filename: &str) -> Draws {
   
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");    

   contents.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>()   
}

fn main() {
   let draws = read_draw_numbers("puzzle-draw.txt");
   let mut boards = read_file("puzzle.txt");

   let ans = part_1_and_2(&mut boards, &draws);

   println!("Part 1: {}, Part 2: {}", ans.0, ans.1);  
}
