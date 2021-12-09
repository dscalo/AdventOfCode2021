extern crate file_reader;
use file_reader::read_file;

extern crate geometry;
use geometry::Point;

type Heightmap = Vec<Vec<i32>>;


fn print_heightmaap(heightmap: &Heightmap) {
    for row in heightmap {
        println!("{:?}", row);
    }
}


fn parse_map(s: &str, heightmap: &mut Heightmap) { 
    let numbs = s.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    heightmap.push(numbs)
}

fn part_1(heightmap: &Heightmap) -> i32 {
    let mut low_points = 0;

    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            let h = heightmap[y][x];

            // up
            if (y as i32 - 1) as i32 >=0 && heightmap[y-1][x] <= h {
                continue
            }

            // right
            if x + 1 < heightmap[y].len() && heightmap[y][x+1] <= h {
                continue
            }

            // down
            if y + 1 < heightmap.len() && heightmap[y+1][x] <= h {
                continue
            }

            // left
            if (x as i32 - 1) as i32 >= 0 && heightmap[y][x-1] <= h {
                continue
            }
            low_points += h +1
        }
    }

    low_points
}

fn get_basin_size(start: Point, heightmap: &Heightmap, visited: &mut Vec<Point>) -> i32 {    
    let mut to_visit: Vec<Point> = vec![start];
    let mut size = 0;
 
    while to_visit.len() > 0 {
        let p = to_visit.pop().unwrap();

        let y = p.y;
        let _y = y as usize;
        let x = p.x;
        let _x = x as usize;     

        if !visited.contains(&p) {
            size +=1;
            visited.push(p);
        }
        

         // up
         if y - 1 >= 0 && heightmap[_y -1][_x] != 9 {
            let new_p = Point::new(x,y -1);
            if !visited.contains(&new_p) {
                to_visit.push(new_p);               
            }
        }

        // right
        if _x + 1 < heightmap[_y].len() && heightmap[_y][_x+1] != 9 {
            let new_p = Point::new(x+1,y);
            if !visited.contains(&new_p) {
                to_visit.push(new_p);               
            }
        }

        // down
        if _y + 1 < heightmap.len() && heightmap[_y+1][_x] != 9 {
            let new_p = Point::new(x, y + 1);
            if !visited.contains(&new_p) {
                to_visit.push(new_p);               
            }
        }

        // left
        if x -1 >= 0 && heightmap[_y][_x-1] != 9 {
            let new_p = Point::new(x -1, y);
            if !visited.contains(&new_p) {
                to_visit.push(new_p);               
            }
        }
    }
     size
    }


fn part_2(heightmap: &Heightmap) -> i32 {
    let mut basins: Vec<i32> = Vec::new();
    let mut visited: Vec<Point> =  Vec::new();
    

    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            if heightmap[y][x] != 9 {
                let size = get_basin_size(Point::new(x as i32, y as i32), &heightmap, &mut visited);
                basins.push(size);
            }
        }
    }


    basins.sort();  
    basins[basins.len()-1] * basins[basins.len()-2] * basins[basins.len()-3]
}

fn main() {
    let mut heightmap: Heightmap = Heightmap::new();    
    read_file("puzzle.txt", parse_map , &mut heightmap);
    
   // print_heightmaap(&heightmap);

   
    let ans_1 = part_1(&heightmap);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2(&heightmap);
    println!("Part 2: {}", ans_2);
}
