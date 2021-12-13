extern crate file_reader;
use file_reader::read_file;

use std::collections::HashMap;

type Graph = HashMap<String, Vec<String>>;
type Paths = Vec<Vec<String>>;

#[derive(Clone)]
struct Path {
    path: Vec<String>,
    lowers: HashMap<String, u32>,
    two_lowers: bool,
    end: bool,
}

impl Path {
    fn new(path: Vec<String>) -> Path {
        let mut l: HashMap<String, u32> = HashMap::new();
        let mut two_lowers = false;

        for p in &path {
            if p == &p.to_lowercase() {
                if l.contains_key(p) {
                    two_lowers = true;
                } else {
                    l.insert(p.clone(), 1);
                }
            }
        }
        Path {
            path: path,
            lowers: l,
            two_lowers: two_lowers,
            end: false,
        }
    }

    fn insert(&mut self, s: String) {
        if s == "end".to_string() {
            self.end = true;
            return;
        }

        if s == s.to_lowercase() {
            if self.lowers.contains_key(&s) {
                self.two_lowers = true;
                self.path.push(s.clone());               
                return;
            }

            self.lowers.insert(s.clone(), 1);       
        }

        self.path.push(s.clone());
    }

    fn can_insert(&self, s: &String) -> bool {
        if self.end {
            return false;
        }

        if s == &s.to_lowercase() {
            if self.lowers.contains_key(s) && self.two_lowers {
                return false;
            }     
        }

        true
    }
}

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
                let node = &paths[n][paths[n].len() - 1];
                let mut fp = false;
                let to_clone = paths[n].clone();
                for g in graph.get(node).unwrap() {
                    if g == &start {
                        continue;
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
                        continue;
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

fn part_2(graph: &Graph) -> u32 {
    let start = "start".to_string();
    let mut paths: Vec<Path> = Vec::new();

    let path = Path::new(vec![start.clone()]);

    paths.push(path);

    loop {
        let mut new_paths: Vec<Path> = Vec::new();
        for n in 0..paths.len() {
            if !paths[n].end {
                let node = &paths[n].path[paths[n].path.len() - 1];
                let mut fp = false;
                let to_clone = paths[n].clone();
                for g in graph.get(node).unwrap() {
                    if g == &start {
                        continue;
                    }

                    if to_clone.can_insert(g) {
                        if !fp {
                            fp = true;
                            paths[n].insert(g.clone());
                            continue;
                        }

                        let mut np = to_clone.clone();
                        np.insert(g.clone());
                        new_paths.push(np);
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
        if path.end {
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
