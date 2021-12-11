extern crate file_reader;
use file_reader::read_file;

type Grid = Vec<Vec<i32>>;

fn print_grid(grid: &Grid) {
    for row in grid {
        println!("{:?}", row)
    }
    println!("----------------------------------------");
}

fn parse_grid(s: &str, grid: &mut Grid) {
    let row = s
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    grid.push(row)
}

fn increment_adjacent(grid: &mut Grid, y: usize, x: usize) -> u32 {
    let mut flashes = 0;
    // top left y-1, x-1
    if y as i32 - 1 >= 0 && x as i32 - 1 >= 0 && grid[y - 1][x - 1] > 0 {
        grid[y - 1][x - 1] += 1;
        if grid[y - 1][x - 1] > 9 {
            grid[y - 1][x - 1] = 0;
            flashes += 1;
            flashes += increment_adjacent(grid, y - 1, x - 1)
        }
    }

    // top y-1
    if y as i32 - 1 >= 0 && grid[y - 1][x] > 0 {
        grid[y - 1][x] += 1;
        if grid[y - 1][x] > 9 {
            grid[y - 1][x] = 0;
            flashes += 1;
            flashes += increment_adjacent(grid, y - 1, x)
        }
    }

    // top right y-1 x+1
    if y as i32 - 1 >= 0 && x + 1 < grid[y].len() && grid[y - 1][x + 1] > 0 {
        grid[y - 1][x + 1] += 1;
        if grid[y - 1][x + 1] > 9 {
            grid[y - 1][x + 1] = 0;
            flashes += 1;
            flashes += increment_adjacent(grid, y - 1, x + 1)
        }
    }

    // right x+1
    if x + 1 < grid[y].len() && grid[y][x + 1] > 0 {
        grid[y][x + 1] += 1;
        if grid[y][x + 1] > 9 {
            grid[y][x + 1] = 0;
            flashes += 1;
            flashes += increment_adjacent(grid, y, x + 1)
        }
    }

    // bottom right y+1 x+1
    if y + 1 < grid.len() && x + 1 < grid[y].len() && grid[y + 1][x + 1] > 0 {
        grid[y + 1][x + 1] += 1;
        if grid[y + 1][x + 1] > 9 {
            grid[y + 1][x + 1] = 0;
            flashes += 1;
            flashes += increment_adjacent(grid, y + 1, x + 1)
        }
    }

    // bottom y+1
    if y + 1 < grid.len() && grid[y + 1][x] > 0 {
        grid[y + 1][x] += 1;
        if grid[y + 1][x] > 9 {
            grid[y + 1][x] = 0;
            flashes += 1;
            flashes += increment_adjacent(grid, y + 1, x)
        }
    }

    // bottom left y+1, x-1
    if y + 1 < grid.len() && x as i32 - 1 >= 0 && grid[y + 1][x - 1] > 0 {
        grid[y + 1][x - 1] += 1;
        if grid[y + 1][x - 1] > 9 {
            grid[y + 1][x - 1] = 0;
            flashes += 1;
            flashes += increment_adjacent(grid, y + 1, x - 1)
        }
    }

    // left x-1
    if x as i32 - 1 >= 0 && grid[y][x - 1] > 0 {
        grid[y][x - 1] += 1;
        if grid[y][x - 1] > 9 {
            grid[y][x - 1] = 0;
            flashes += 1;
            flashes += increment_adjacent(grid, y, x - 1)
        }
    }

    flashes
}

fn increase_energy_levels(grid: &mut Grid) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x] += 1;
        }
    }
}

fn step(grid: &mut Grid) -> u32 {
    let mut flashes = 0;
    increase_energy_levels(grid);

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] > 9 {
                grid[y][x] = 0;
                flashes += 1;
                flashes += increment_adjacent(grid, y, x)
            }
        }
    }
    flashes
}

fn part_1(grid: &mut Grid, steps: u32) -> u32 {
    let mut flashes = 0;
    for _ in 0..steps {
        flashes += step(grid);
    }

    flashes
}

fn part_2(grid: &mut Grid) -> u32 {
    let mut counter = 0;

    loop {
        let flashes = step(grid);
        counter += 1;
        if flashes == 100 {
            break;
        }
    }

    counter
}

fn main() {
    let mut grid: Grid = Grid::new();

    read_file("puzzle.txt", parse_grid, &mut grid);

    let mut p2_grid = grid.clone();
    //print_grid(&grid);

    let ans_1 = part_1(&mut grid, 100);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2(&mut p2_grid);
    println!("Part 2: {}", ans_2);
}
