#[derive(Debug, Clone)]
struct Probe {
    x: i64,
    y: i64,
    x_vel: i64,
    y_vel: i64,
}

impl Probe {
    fn new(x: i64, y: i64) -> Probe {
        Probe {
            x: 0,
            y: 0,
            x_vel: x,
            y_vel: y,
        }
    }

    fn step(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;

        if self.x_vel > 0 {
            self.x_vel -= 1;
        } else if self.x_vel < 0 {
            self.x_vel += 1;
        }

        self.y_vel -= 1;
    }
}

#[derive(Debug)]
struct Target {
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}

impl Target {
    fn new(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Target {
        Target {
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Shot {
    Hit(i64),
    Miss(i64),
}

fn step_until(target: &Target, probe: &Probe) -> Shot {
    let mut p = probe.clone();
    let mut max_y = i64::MIN;

    loop {
        let prev_y = p.y;
        p.step();
        if p.y > max_y {
            max_y = p.y
        }

        if p.x > target.max_x {
            // over shot x
            return Shot::Miss(0);
        }

        if p.y < target.min_y {
            // over shot y
            //println!("OVER SHOT Y BY {} - {} = {} ||| prev_y = {}, y_vel = {}", target.min_y, p.y, target.min_y - p.y, prev_y, p.y_vel);
            return Shot::Miss(target.min_y - p.y);
        }

        if p.x >= target.min_x && p.x <= target.max_x && p.y <= target.max_y && p.y >= target.min_y
        {
            // hit target area
            return Shot::Hit(max_y);
        }

        // if breaker > 100 {
        //     println!("SOMETHIMG IS WRONG!!!!!!");
        //     break;
        // }
    }
}

// because i'm bad at math
fn find_min_x_velocity(target_min_x: i64) -> i64 {
    let mut min_vel = 0;
    loop {
        let mut x = 0;
        min_vel += 1;
        let mut test_vel = min_vel;
        while test_vel > 0 {
            x += test_vel;
            test_vel -= 1;
        }

        if x >= target_min_x {
            return min_vel;
        }
    }
}

fn find_min_y_velocity(min_x: i64, target: &Target) -> i64 {
    let mut min_y_vel = 0;
    let mut probe = Probe::new(min_x, min_y_vel);

    let mut res = step_until(&target, &probe);

    loop {
        match res {
            Shot::Hit(_) => break,
            Shot::Miss(_) => {
                min_y_vel += 1;
                probe = Probe::new(min_x, min_y_vel);
                res = step_until(&target, &probe);
            }
        }
    }

    min_y_vel
}

fn part_1(target: &Target) -> i64 {
    let min_x = find_min_x_velocity(target.min_x);
    let min_y = find_min_y_velocity(min_x, &target);
    let mut largest_y = 0;
    let mut best_x_y = (min_x, min_y);

    println!("min x vel is {}, min y vel is {}", min_x, min_y);

    for try_x in min_x..=target.max_x {
        //println!("TRYING X: {}", try_x);
        let mut try_y = min_y;
        let mut probe = Probe::new(try_x, try_y);
        let mut try_again = 10000;
        loop {
            //println!("Probe y {}", probe.y_vel);
            let res = step_until(&target, &probe);
            match res {
                Shot::Hit(res_y) => {
                    if res_y > largest_y {
                        largest_y = res_y;
                        best_x_y = (try_x, try_y)
                    }
                    try_y += 1;
                    probe = Probe::new(try_x, try_y);
                    try_again = 10000;
                }
                Shot::Miss(missed_by) => {
                    if missed_by != 0 {
                        //println!("missed By {}", missed_by);
                        if try_again > 0 {
                            try_y += 1;
                            //println!("TRYING AGAIN AY Y: {}", try_y);
                            probe = Probe::new(try_x, try_y);

                            try_again -= 1;
                        } else {
                            //println!("BREAK AFTER TY AGAIN");
                            break;
                        }
                    } else {
                        //println!("BREAK");
                        break;
                    }
                }
            }
        }
    }

    println!("Largest Y: {}, best: {:?}", largest_y, best_x_y);
    // 1275 too low
    largest_y
}

fn part_2(target: &Target) -> i64 {
    let min_x = find_min_x_velocity(target.min_x);
    let min_y = target.min_y;
    let mut hits = 0;

    println!("min x vel is {}, min y vel is {}", min_x, min_y);

    for try_x in min_x..=target.max_x {
        //println!("TRYING X: {}", try_x);
        let mut try_y = min_y;
        let mut probe = Probe::new(try_x, try_y);
        let mut try_again = 10000;
        loop {
            //println!("Probe y {}", probe.y_vel);
            let res = step_until(&target, &probe);
            match res {
                Shot::Hit(_) => {
                    hits += 1;
                    try_y += 1;
                    probe = Probe::new(try_x, try_y);
                    try_again = 10000;
                }
                Shot::Miss(missed_by) => {
                    if missed_by != 0 {
                      
                        if try_again > 0 {
                            try_y += 1;
                            
                            probe = Probe::new(try_x, try_y);

                            try_again -= 1;
                        } else {
                            //println!("BREAK AFTER TY AGAIN");
                            break;
                        }
                    } else {
                        //println!("BREAK");
                        break;
                    }
                }
            }
        }
    }
  
    hits
}

fn main() {
    let test = Target::new(20, 30, -10, -5);
    let puzzle = Target::new(14, 50, -267, -225);

    let ans_1 = part_1(&puzzle);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2(&puzzle);
    println!("Part 2: {}", ans_2);
}
