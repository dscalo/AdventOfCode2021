extern crate file_reader;
use file_reader::read_file;

use std::collections::HashMap;
use std::collections::VecDeque;

type Graph = HashMap<String, Vec<String>>;
type Paths = Vec<Vec<String>>;

fn print_graph(graph: &Graph) {
    for (k, v) in graph {
        println!("{} - {:?}", k, v)
    }
}

fn parse_graph(s: &str, graph: &mut Graph) {
    let row = s.split("-").collect::<Vec<&str>>();

    if graph.contains_key(row[0]) && !graph.get(row[0]).unwrap().contains(&row[1].to_string()) {
        graph.get_mut(row[0]).unwrap().push(row[1].to_string())
    } else {
        let v = vec![row[1].to_string()];
        graph.insert(row[0].to_string(), v);
    }

    if graph.contains_key(row[1]) && !graph.get(row[0]).unwrap().contains(&row[0].to_string()) {
        graph.get_mut(row[1]).unwrap().push(row[0].to_string())
    } else {
        let v = vec![row[0].to_string()];
        graph.insert(row[1].to_string(), v);
    }
}

fn is_uppercase(s: &String) -> bool {
    s == &s.to_uppercase()
}


fn part_1(graph: &Graph) -> u32 {
    let end = "end".to_string();
    let start = "start".to_string();
    let mut paths: Paths = Paths::new();

    paths.push(vec![start.clone()]);

    loop {
        let mut new_paths: Paths = Paths::new();
        for n in 0..paths.len() {            
            if !paths[n].contains(&end) {
                let node = &paths[n][paths[n].len()-1];
               
                let mut fp = false;
                let to_clone =  paths[n].clone();
                for g in graph.get(node).unwrap() {
                    if g == &start {
                        continue
                    }
                   
                    if is_uppercase(g) || g == &end || !paths[n].contains(g) {

                        if !fp {
                            fp = true;
                            paths[n].push(g.clone());
                            continue;

                        }
                        let mut np = to_clone.clone();
                        np.push(g.clone());
                        new_paths.push(np);
                        // println!(" new_paths::{:?}", new_paths);
                        // println!("_________________________________");
                        continue
                    }
                    
                }
            }
        }
        if new_paths.len() == 0 {
            break;
        }

        paths.append(&mut new_paths);
        //  println!(" PATHS::{:?}", paths);
        // println!("_________________________________");     
       
    }

    let mut counter = 0;
    
    for path in paths {
        //println!(" PATH::{:?}", path);
        if path.contains(&end) {
            counter += 1;
        }
    }
    counter
}

fn small_cave_twice(path: &Vec<String>) -> bool {
    for p in path {
        if p != &p.to_lowercase() {
            continue;
        }
        if path.iter().filter(|s| s == &p).count() > 1 {
           
            return true
        }
    }

    false
}

fn part_2(graph: &Graph) -> u32 {
    let end = "end".to_string();
    let start = "start".to_string();
    let mut paths: Paths = Paths::new();

    paths.push(vec![start.clone()]);

    loop {
        let mut new_paths: Paths = Paths::new();
        for n in 0..paths.len() {            
            if !paths[n].contains(&end) {
                let node = &paths[n][paths[n].len()-1];
               
                let mut fp = false;
                let to_clone = paths[n].clone();
                for g in graph.get(node).unwrap() {
                    if g == &start {
                        continue
                    }
                   
                    if is_uppercase(g) || g == &end || !to_clone.contains(g) || (to_clone.contains(g) && !small_cave_twice(&to_clone)) {

                        if !fp {
                            fp = true;
                            paths[n].push(g.clone());
                            continue;

                        }
                        let mut np = to_clone.clone();
                        np.push(g.clone());
                        new_paths.push(np);                       
                        continue
                    }
                    
                }
            }
        }
        if new_paths.len() == 0 {
            break;
        }

        paths.append(&mut new_paths);      
       
    }

    let mut counter = 0;
    
    for path in paths {
        if path.contains(&end) {
            counter += 1;
        }
    }
    counter
}

fn main() {
    let mut graph: Graph = Graph::new();
    read_file("puzzle.txt", parse_graph, &mut graph);

    //print_graph(&graph);
    let ans_1 = part_1(&graph);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2(&graph);
    println!("Part 2: {}", ans_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_uppercase_works() {
        assert_eq!(is_uppercase(&"GG".to_string()), true);
        assert_eq!(is_uppercase(&"Z".to_string()), true);
        assert_eq!(is_uppercase(&"Gg".to_string()), false);
        assert_eq!(is_uppercase(&"b".to_string()), false);
    }
}
