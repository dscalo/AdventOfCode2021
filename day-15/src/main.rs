extern crate file_reader;
use file_reader::read_file;

extern crate geometry;
use geometry::Point;

use std::collections::HashMap;

type Grid = Vec<Vec<i32>>;
type Visited = HashMap<Point, bool>;
type Distance = HashMap<Point, i32>;

fn print_grid(grid: &Grid) {
   for y in 0..grid.len(){
       for x in 0..grid[y].len() {
           print!("{}", grid[y][x])
       }
       println!("");
   }
   println!("_______________________________________________");
}

fn parse_grid(s: &str, grid: &mut Grid) {
    let row = s
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    grid.push(row)
}

fn get_val(grid: &Grid, point: &Point) -> i32 {
    let uy = point.y as usize;
    let ux = point.x as usize;
    grid[uy][ux]
}

fn get_dist_val(dist: &Distance, point: &Point) -> i32 {
    if !dist.contains_key(&point) {
        return i32::MAX;
    }

    *dist.get(&point).unwrap()
}

fn get_end_point(grid: &Grid) -> Point {
    let end_y = (grid.len() - 1) as i32;
    let end_x = (grid[0].len() -1) as i32;
    Point::new(end_x, end_y)
}

fn get_min_distance(dist: &Distance, visited: &Visited) -> Point {
    let mut min = i32::MAX;
    let mut point = Point::new(0,0);
   for (k,v) in dist {
    if v < &min && !visited.contains_key(&k) {
        min = *v;
        point = k.clone()
    }
   }

    point
}

fn dijsktra(grid: &Grid) -> i32 {
    let mut dist: Distance = Distance::new();
    let mut visited: Visited = Visited::new();
    let start = Point::new(0, 0);
    let end = get_end_point(&grid);   

    dist.insert(start.clone(), 0);
    
    loop {
        let pos = get_min_distance(&dist, &visited);

        visited.insert(pos.clone(), true);

        if pos == end {
            break;
        }       

        let up = Point::new(pos.x, pos.y -1);
        if up.y >= 0 && !visited.contains_key(&up) {
            let ud = get_dist_val(&dist, &up);
            let uc = get_val(&grid, &up);
            let cd = get_dist_val(&dist, &pos);

            if  cd < i32::MAX && cd + uc < ud  {
                dist.insert(up, cd + uc);               
            }

        }

        let right = Point::new(pos.x+1, pos.y);
        if right.x < grid[0].len() as i32 && !visited.contains_key(&right) {
            let ud = get_dist_val(&dist, &right);
            let uc = get_val(&grid, &right);
            let cd = get_dist_val(&dist, &pos);            

            if  cd < i32::MAX && cd + uc < ud  {
                dist.insert(right, cd + uc);   
            }

        }

        let down = Point::new(pos.x, pos.y +1);
        if down.y < grid.len() as i32 && !visited.contains_key(&down) {
            let ud = get_dist_val(&dist, &down);
            let uc = get_val(&grid, &down);
            let cd = get_dist_val(&dist, &pos);

            if cd < i32::MAX &&  cd + uc < ud  {
                dist.insert(down, cd + uc);   
            }

        }
        
        let left = Point::new(pos.x-1, pos.y);
        if left.x  >= 0 && !visited.contains_key(&left) {
            let ud = get_dist_val(&dist, &left);
            let uc = get_val(&grid, &left);
            let cd = get_dist_val(&dist, &pos);

            if cd < i32::MAX && cd + uc < ud  {
                dist.insert(left, cd + uc);   
            }

        }  
        
        dist.remove_entry(&pos);       
    }
    // print_grid(&dist);
    // println!("{:?}", visited);  

    get_dist_val(&dist, &end)
}

fn part_1(grid: &Grid) -> i32 {
   dijsktra(&grid)
}

fn part_2(grid: &Grid) -> i32 {
    let mut big_grid = grid.clone();
    let y_len = grid.len();

    // increase y * 5
    for _y in 0..4 {
        let start = _y * y_len;
          for y in start..y_len + start {
            let mut row: Vec<i32> = Vec::new();
            for x in 0..big_grid[y].len() {
                let mut v = big_grid[y][x] + 1;
                if v > 9 {
                    v = 1
                }
                row.push(v);
    
            }
            big_grid.push(row)
        }
    }

    // increase x * 5
    for y in 0..big_grid.len() {
        for x in 0..big_grid.len() -y_len{
            let mut val = big_grid[y][x] + 1;
            if val > 9 {
                val = 1
            }
            big_grid[y].push(val);
        }
    }    

    dijsktra(&big_grid)
}

fn main() {
    let mut grid:Grid = Grid::new();
    read_file("puzzle.txt", parse_grid , &mut grid);

    let ans_1 = part_1(&grid);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2(&grid);
    println!("Part 2: {}", ans_2);
}
