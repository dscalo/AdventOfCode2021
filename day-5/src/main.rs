extern crate file_reader;
use file_reader::read_file;

mod geometry;
use crate::geometry::{Line, Point};

type Lines = Vec<Line>;
type Grid = Vec<Vec<u32>>;


fn get_point(s: &str) -> Point {
    let xy = s.split(",").collect::<Vec<&str>>();

    let x = xy[0].parse::<i32>().unwrap();
    let y = xy[1].parse::<i32>().unwrap();

    Point::new(x,y)
}

fn print_grid(grid: &Grid) {
    for row in grid {
        println!("{:?}", row)
    }
}
 

fn parse_lines(s: &str, lines: &mut Lines) { 
    // 2,2 -> 2,1
    let ls = s.split(" -> ").collect::<Vec<&str>>();

    let line = Line::new(get_point(ls[0]),get_point(ls[1]));

    lines.push(line);
}

fn get_max_x_y(lines: &Lines) -> Point {
    let mut x = 0;
    let mut y = 0;

    for line in lines {
        if line.start.x > x {
            x = line.start.x
        }

        if line.end.x > x {
            x = line.end.x
        }

        if line.start.y > y {
            y = line.start.y
        }

        if line.end.y > y {
            y = line.end.y
        }
    }


    Point::new(x +1,y + 1)
}

fn count_overlaps(grid: &Grid) -> i32 {
    let mut overlaps = 0;
    for row in grid {
        for r in row {
            if r > &1 {
                overlaps += 1
            }
        }
    }

    overlaps
}

fn part_1_or_2(lines: &Lines, max: &Point, skip_diagonals: bool) -> i32 {
    let mut grid:Grid = Grid::new();

    for _ in 0..max.y {
        let mut row: Vec<u32> = Vec::new();
        for _ in 0..max.x {
            row.push(0)
        }
        grid.push(row)
    }

    //print_grid(&grid);

    for line in lines {
        match (line.start.x - line.end.x, line.start.y - line.end.y, skip_diagonals) {
            (0,_,_) => {
                // vert
                grid[line.end.y as usize][line.end.x as usize] += 1;
                let mut m = 1;
                let mut y = line.start.y;
                if line.end.y < line.start.y {
                    m = -1
                }
    
                while y != line.end.y {
                    grid[y as usize][line.start.x as usize] += 1;
                    y = y+ m;
                }
            },
            (_,0,_) => {
                // hoz
                grid[line.end.y as usize][line.end.x as usize] += 1;
                let mut m = 1;
                let mut x = line.start.x;
    
                if line.end.x < line.start.x {
                    m = -1
                }
    
                while x != line.end.x {
                    grid[line.start.y as usize][x as usize] += 1;
                    x = x + m;
                }

            },
            (_,_, false) => {
                // diag
                grid[line.end.y as usize][line.end.x as usize] += 1;
                let mut dx = 1;
                let mut dy = 1;
                let mut x = line.start.x;
                let mut y = line.start.y;

                if line.end.y < line.start.y {
                    dy = -1
                }

                if line.end.x < line.start.x {
                    dx = -1
                }

                while y != line.end.y {
                    grid[y as usize][x as usize] +=1;
                    y = y + dy;
                    x = x + dx;
                }
            },
            _ => {}
        }
    }

    //print_grid(&grid);

    count_overlaps(&grid)
}

fn main() {
    let mut lines: Lines = Lines::new();

    read_file("puzzle.txt", parse_lines, &mut lines);

    let max_point = get_max_x_y(&lines);

    let ans_1 = part_1_or_2(&lines, &max_point, true);
    println!("Part 1: {}", ans_1);    

    let ans_2 = part_1_or_2(&lines, &max_point, false);
    println!("Part 2: {}", ans_2);
}
