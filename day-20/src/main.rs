extern crate file_reader;
use file_reader::read_file;
use geometry::Point;
use std::fs;

type Grid = Vec<Vec<char>>;
type IEA = Vec<char>; // image enhancement algorithm

fn print_grid(grid: &Grid) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", grid[y][x])
        }
        println!("");
    }
    println!("                        *****                                 ");
}

fn parse_grid(s: &str, grid: &mut Grid) {
    let row = s.chars().collect::<Vec<char>>();
    grid.push(row)
}

fn read_iea(filename: &str) -> IEA {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    contents.chars().collect::<Vec<char>>()
}

fn expand_grid(grid: &Grid, pad_with: char) -> Grid {
    let mut ex_grid: Grid = Grid::new();

    let y_pad = 2; //grid.len();
    let l_pad = 2; // grid[0].len();
    let new_x = grid[0].len() + l_pad * 2;
    // top pad
    for _ in 0..y_pad {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..new_x {
            row.push(pad_with);
        }
        ex_grid.push(row)
    }

    for y in 0..grid.len() {
        let mut row: Vec<char> = Vec::new();
        // left pad
        for _ in 0..l_pad {
            row.push(pad_with);
        }

        for x in 0..grid[y].len() {
            row.push(grid[y][x]);
        }

        //right pad
        for _ in 0..l_pad {
            row.push(pad_with);
        }

        ex_grid.push(row)
    }

    // bottom pad
    for _ in 0..y_pad {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..new_x {
            row.push(pad_with);
        }
        ex_grid.push(row)
    }

    ex_grid
}

fn trim_grid(grid: &Grid) -> Grid {
    let mut t_grid: Grid = Grid::new();

    let mut min_x = grid[0].len();
    let mut max_x = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '#' {
                if x > max_x {
                    max_x = x;
                }

                if x < min_x {
                    min_x = x;
                }
            }
        }
    }

    for y in 0..grid.len() {
        if !grid[y].contains(&'#') {
            continue;
        }
        let mut row: Vec<char> = Vec::new();
        for x in min_x..=max_x {
            row.push(grid[y][x]);
        }
        t_grid.push(row)
    }
    t_grid
}

fn blank_grid(y_len: usize, x_len: usize) -> Grid {
    let mut grid: Grid = Grid::new();

    for _ in 0..y_len {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..x_len {
            row.push('.');
        }
        grid.push(row)
    }

    grid
}

fn code_to_binary(code: &Vec<char>) -> String {
    let mut binary_str = "".to_string();

    for c in code {
        match c {
            '.' => binary_str.push('0'),
            '#' => binary_str.push('1'),
            _ => panic!("Invalid code!"),
        }
    }

    binary_str
}

fn binary_to_int(s: &str) -> i64 {
    i64::from_str_radix(&s, 2).unwrap()
}

fn part_1_and_2(grid: &Grid, iea: &IEA, enhance: usize) -> i64 {
    let mut pad_with = '.';
    let mut cur = expand_grid(grid, pad_with);

    let mut inf_char = iea[0];
    let mut lit_count = 0;

    for _ in 0..enhance {
        lit_count = 0;
        let mut next = blank_grid(cur.len(), cur[0].len());

        for y in 1..cur.len() - 1 {
            for x in 1..cur[y].len() - 1 {
                let mut code: Vec<char> = Vec::new();
                // get top row y-1,x-1 | y-1,x | y-1,x+1
                code.push(cur[y - 1][x - 1]);
                code.push(cur[y - 1][x]);
                code.push(cur[y - 1][x + 1]);

                // get mid row y,x-1 | y,x | y,x+1
                code.push(cur[y][x - 1]);
                code.push(cur[y][x]);
                code.push(cur[y][x + 1]);

                // get bottom row y+1,x-1 | y+1,x | y+1,x+1
                code.push(cur[y + 1][x - 1]);
                code.push(cur[y + 1][x]);
                code.push(cur[y + 1][x + 1]);
                let binary_str = code_to_binary(&code);
                let lookup = binary_to_int(&binary_str);

                if iea[lookup as usize] == '#' {
                    lit_count += 1
                }
                next[y][x] = iea[lookup as usize];
            }
        }
        if inf_char == '#' {
            pad_with = '#';
            inf_char = '.'
        } else {
            inf_char = iea[0];
            pad_with = '.';
        }
        cur = expand_grid(&trim_grid(&next), pad_with);
    }
    lit_count
}

fn main() {
    let filename = "puzzle".to_string();

    let mut grid: Grid = Grid::new();

    read_file(&format!("{}.txt", filename), parse_grid, &mut grid);
    let iea = read_iea(&format!("{}-iea.txt", filename));

    let ans_1 = part_1_and_2(&grid, &iea, 2);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_1_and_2(&grid, &iea, 50);
    println!("Part 2: {}", ans_2);
}
