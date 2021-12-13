extern crate file_reader;
use file_reader::read_file;

extern crate geometry;
use geometry::Point;

type Points = Vec<Point>;
type Grid = Vec<Vec<char>>;

fn parse_points(s: &str, points: &mut Points) {
    let row = s.split(",").collect::<Vec<&str>>();
    let x = row[0].parse::<i32>().unwrap();
    let y = row[1].parse::<i32>().unwrap();
    points.push(Point::new(x, y));
}

fn parse_folds(s: &str, points: &mut Points) {
    let row = s.split("=").collect::<Vec<&str>>();

    let mut y = 0;
    let mut x = 0;
    if row[0] == "y" {
        y = row[1].parse::<i32>().unwrap();
    } else {
        x = row[1].parse::<i32>().unwrap();
    }
    points.push(Point::new(x, y));
}

fn print_grid(grid: &Grid) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", grid[y][x])
        }
        println!("");
    }
    println!("_________________________________________________________________");
}

fn get_max_x_y(points: &Points) -> Point {
    let mut max_x_y = Point::new(0, 0);

    for point in points {
        if point.x > max_x_y.x {
            max_x_y.x = point.x
        }

        if point.y > max_x_y.y {
            max_x_y.y = point.y;
        }
    }

    max_x_y.x += 1;
    max_x_y.y += 1;

    max_x_y
}

fn fill_grid(grid: &mut Grid, xy: &Point) {
    for _ in 0..xy.y {
        let mut row: Vec<char> = Vec::new();

        for _ in 0..xy.x {
            row.push('.')
        }
        grid.push(row);
    }
}

fn count_grid(grid: &Grid) -> u32 {
    let mut counter = 0;
    for row in grid {
        for r in row {
            if r == &'#' {
                counter += 1
            }
        }
    }
    counter
}

fn mark_grid(grid: &mut Grid, point: &Point, mark: char) {
    grid[point.y as usize][point.x as usize] = mark;
}

fn create_grid(points: &Points) -> Grid {
    let mut grid: Grid = Grid::new();

    let max_x_y = get_max_x_y(points);

    fill_grid(&mut grid, &max_x_y);

    for point in points {
        mark_grid(&mut grid, &point, '#')
    }

    grid
}

fn fold_points(points: &mut Points, fold: &Point) {
    if fold.x == 0 {
        // fold along y
        for p in 0..points.len() {
            if points[p].y > fold.y {
                points[p].y = fold.y - ( points[p].y -fold.y)
            }
        }

        return;
    }

    // fold along x
    for p in 0..points.len() {
        if points[p].x > fold.x {
            points[p].x = fold.x - ( points[p].x -fold.x)
        }
    }
}

fn part_1(points: &mut Points, folds: &Points) -> u32 {
    fold_points(points, &folds[0]);
    let grid = create_grid(points);

    //print_grid(&grid);
    count_grid(&grid)
}

fn part_2(points: &mut Points, folds: &Points) {  
    for fold in folds {
        fold_points(points, &fold);
    }
   
    let grid = create_grid(points);

    print_grid(&grid);   
}

fn main() {
    let mut points: Points = Points::new();
    let mut folds: Points = Points::new();
    read_file("puzzle.txt", parse_points, &mut points);
    read_file("puzzle-fold.txt", parse_folds, &mut folds);

    let mut points_p2 = points.clone();

    let ans_1 = part_1(&mut points, &folds);
    println!("Part 1: {}", ans_1);

    part_2(&mut points_p2, &folds);
}
