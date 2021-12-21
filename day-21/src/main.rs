use std::collections::HashMap;

fn move_player(p: i64, mov: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    let mut n = p + mov;

    if cache.contains_key(&n) {
        return *cache.get(&n).unwrap();
    }

    let op = n;

    if n > 10 {
        n = n % 10;
    }

    if n == 0 {
        n = 10;
    }

    cache.insert(op, n);

    n
}

struct Die {
    die: i64,
    sides: i64,
    rolled: i64,
}

impl Die {
    fn new(start: i64, max: i64) -> Die {
        Die {
            die: start,
            sides: max,
            rolled: 0,
        }
    }

    fn roll(&mut self, times: usize) -> i64 {
        self.rolled += times as i64;
        let mut val = 0;

        for _ in 0..times {
            self.die += 1;
            if self.die > self.sides {
                self.die = 1;
            }
            //print!(" {} ",self.die);
            val += self.die;
        }
        //println!();
        val
    }
}

fn part_1(p1: i64, p2: i64) -> i64 {
    let mut die = Die::new(0, 100);
    let mut cache: HashMap<i64, i64> = HashMap::new();

    let mut p1_pos = p1;
    let mut p2_pos = p2;
    let mut p1_s = 0;
    let mut p2_s = 0;

    loop {
        p1_pos = move_player(p1_pos, die.roll(3), &mut cache);
        p1_s += p1_pos;
        if p1_s >= 1000 {
            return p2_s * die.rolled;
        }

        // player 2's turn
        p2_pos = move_player(p2_pos, die.roll(3), &mut cache);
        p2_s += p2_pos;
        if p2_s >= 1000 {
            return p1_s * die.rolled;
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Game {
    p1: u32,
    p2: u32,
    s1: u32,
    s2: u32,
    t: u32,
    r: u32,
}

fn move_p(p: (u32, u32), a: u32) -> (u32, u32) {
    let mut n = p.0 + a;
    if n > 10 {
        n = n % 10;
    }

    if n == 0 {
        n = 10;
    }
    (n, p.1 + n)
}

#[derive(Debug)]
struct Wins(i64, i64);

fn play_2(
    p1: (u32, u32),
    p2: (u32, u32),
    cache: &mut HashMap<((u32, u32), (u32, u32)), (u64, u64)>,
) -> (u64, u64) {
    let mut wins: (u64, u64) = (0, 0);
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                let np = move_p(p1, a + b + c);
                if np.1 >= 21 {
                    wins.0 += 1
                } else {
                    if cache.contains_key(&(p2, np)) {
                        let w = cache.get(&(p2, np)).unwrap();
                        wins.0 += w.1;
                        wins.1 += w.0;
                    } else {
                        let w = play_2(p2, np, cache);
                        wins.0 += w.1;
                        wins.1 += w.0;
                        cache.insert((p2, np), w);
                    }
                }
            }
        }
    } 
    wins
}

fn part_2_v2(p1: u32, p2: u32) -> u64 {
    let mut cache: HashMap<((u32, u32), (u32, u32)), (u64, u64)> = HashMap::new();
    let wins = play_2((p1, 0), (p2, 0), &mut cache);

    if wins.0 > wins.1 {
        return wins.0
    }
    wins.1
}

fn main() {
    /*
        TEST
            Player 1 starting position: 4
            Player 2 starting position: 8

        PUZZLE
            Player 1 starting position: 1
            Player 2 starting position: 3

    */

    let ans_1 = part_1(1,3);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2_v2(1,3);
    println!("Part 2: {}", ans_2);
}
