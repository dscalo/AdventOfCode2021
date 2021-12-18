extern crate file_reader;
use file_reader::read_file;
use std::collections::VecDeque;

type Numbers = Vec<NumberLine>;
type NumberLine = VecDeque<i64>;

fn print_number_line(number_line: &NumberLine) {
    for n in number_line {
        match n {
            -1 => print!("["),
            -2 => print!("]"),
            -3 => print!(","),
            _ => print!("{}", n)
        }            
    }
    println!("");
}

fn print_numbers(numbers: &Numbers) {
    for number in numbers {
        print_number_line(number)
    }
    println!("------------------------------------------");
}

fn create_number_line(chars: &Vec<char>) -> NumberLine {
    let mut number_line:NumberLine = NumberLine::new();
    for c in chars {       
        match c {
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                number_line.push_back(c.to_digit(10).unwrap() as i64)
            },
            '[' => number_line.push_back(-1),
            ']' => number_line.push_back(-2),
            ',' => number_line.push_back(-3),
            _ =>{}
        }
    }
    number_line
}

// [ = -1   ] = -2  , = -3 because i don't want to do conversions all night
fn parse_numbers(s: &str, numbers: &mut Numbers) {
    let row = s.chars().collect::<Vec<char>>();   
    numbers.push(create_number_line(&row))
}

fn explode(start: usize, nl: &NumberLine) -> NumberLine {
    let mut new_nl: NumberLine= NumberLine::new();
    //  s  1   2  3  4
    // -1  X  -3  Y -2
    let x = nl[start + 1];
    let y = nl[start + 3];
    //println!("x: {}, y: {}", x,y);

    let mut n = start;
    let mut fln_idx = 0;

    while n > 0 {
        n = n -1;
        if nl[n] >= 0 {
            fln_idx = n;
            break;
        }
    }

    let mut added_y = false;
    for i in 0..nl.len() {
        if i > 0 && i == fln_idx {
            new_nl.push_back(nl[fln_idx] + x);
            continue;
        }

        if i < start {
            new_nl.push_back(nl[i]);
            continue;
        }
         if i == start {
             new_nl.push_back(0);
             continue;
         }

         if i <= start + 4 {
             continue;
         }

         // past the end of the exploded pair
         if nl[i] >= 0 && !added_y {
             new_nl.push_back(nl[i] + y);
             added_y = true;
             continue;
         }

         new_nl.push_back(nl[i])
    }


    new_nl
}

fn divide_into_pair(numb: i64) -> (i64,i64) {
    let f_num = numb as f32;

    let x = (f_num / 2.0).floor() as i64;
    let y = (f_num / 2.0).ceil() as i64;

    (x,y)
}

fn split_pair(start: usize, number_line: &NumberLine) -> NumberLine {
    let mut nl: NumberLine = NumberLine::new();

    //  s  1   2  3 
    //  X  -3  Y -2   
    
    let (x,y) = divide_into_pair(number_line[start]);

    for n in 0..start {
        nl.push_back(number_line[n])
    }
    

    nl.append(&mut VecDeque::from(vec![-1,x,-3,y,-2]));

    for n in start+1..number_line.len() {
        nl.push_back(number_line[n]);
    }
    nl

}

fn reduce(number_line: &NumberLine) -> NumberLine {
    let mut nl = number_line.clone();  
   
    let mut can_reduce = true;

    while can_reduce {
        can_reduce = false;
        let mut depth = 0;
        let mut n = 0;

        // check for explodes
        // 1. if any pair is nested inside 4 pairs -> explode
        while n < nl.len() {
            match nl[n] {
                -1 => depth += 1,
                -2 => depth -= 1,
                _ => {}
            }       
    
            if depth >= 5 && nl[n] >= 0 {
               //println!("Calling Explode! start is : {}", n);
                nl = explode(n-1, &mut nl);
                //println!("AFTER EXPLODE:");
                //print_number_line(&nl);
                n = 0;
                depth = 0;
                can_reduce = true;
                continue;
            }
            n += 1;   
        }

        n = 0;

         // check for splits 
        // 2. if any number > 10 -> split
        while n < nl.len() {
            if nl[n] >= 10 {
                //println!("Calling split!!! start: {}", n);

                nl = split_pair(n, &nl);
                //println!("AFTER SPLIT:");
                //print_number_line(&nl);
                can_reduce = true;
                break;
            }
            n += 1;
        }        
    }  

   nl
}



fn magnitude(number_line: &NumberLine) -> i64 {
    let mut nl: NumberLine =number_line.clone();
   
    while nl.len() > 1 {
        let mut tmp: NumberLine = NumberLine::new();
        let mut n = 0;
        while n < nl.len() {
           tmp.push_back(nl[n]);
            if n+3 < nl.len() && nl[n+1] >= 0 && nl[n+3]  >= 0 {
                // pair of numbers
                tmp.pop_back();
                tmp.push_back((nl[n+1] * 3) +  (nl[n+3] * 2));
                n += 5
            } else {
                n += 1
            }           
        }
        //print_number_line(&tmp);
        nl = tmp;  
        
    }
    nl[0]
}

fn add_number_lines(nl1: &NumberLine, nl2: &NumberLine) -> NumberLine {
    let mut number_line: NumberLine = NumberLine::new();
    
    number_line.append(&mut nl1.clone());

    number_line.push_front(-1);
    number_line.push_back(-3);
    number_line.append(&mut nl2.clone());
    number_line.push_back(-2);

    number_line
}

fn part_1(numbers: &Numbers) -> i64 {
    let mut number_line: NumberLine = NumberLine::new();
  
    number_line.append(&mut numbers[0].clone());

    print_number_line(&number_line);

    for n in 1..numbers.len() {
        number_line = add_number_lines(&number_line, &numbers[n]);
        //print_number_line(&number_line);
        number_line = reduce(&number_line);
        //print_number_line(&number_line);
    }

    magnitude(&number_line)
}


fn part_2(numbers: &Numbers) -> i64 {
    let mut lg_mag = 0;

    for n in 0..numbers.len() {
        for m in 0..numbers.len() {
            if n == m {
                continue;
            }

            let mut tmp = magnitude(&reduce(&add_number_lines(&numbers[n], &numbers[m])));
            if tmp > lg_mag {
                lg_mag = tmp
            }

            tmp = magnitude(&reduce(&add_number_lines(&numbers[m], &numbers[n])));

            if tmp > lg_mag {
                lg_mag = tmp
            }

        }
    }

    lg_mag
}

fn reduce_test() {
    let chars =  "[[[[[9,8],1],2],3],4]".chars().collect::<Vec<char>>();                
    let chars2 = "[7,[6,[5,[4,[3,2]]]]]".chars().collect::<Vec<char>>();
    let mut chars3 = "[[[[0,7],4],[15,[0,13]]],[1,1]]".chars().collect::<Vec<char>>();

    let mut test = create_number_line(&chars);
    print_number_line(&test);
    test = reduce(&test);
    print_number_line(&test);
    println!("----------------------------------------------------");

    let mut test2 = create_number_line(&chars2);
    print_number_line(&test2);
    test2 = reduce(&test2);
    print_number_line(&test2);
    println!("----------------------------------------------------");

    let mut split_test = create_number_line(&chars3);
    print_number_line(&split_test);
    split_test = split_pair(13, &split_test);
    print_number_line(&split_test);
    
}

fn magnitude_test() {
    println!("--------------- magnitude test ---------------");
    let chars = "[[1,2],[[3,4],5]]".chars().collect::<Vec<char>>();
    let chars2 = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".chars().collect::<Vec<char>>();

    let test = create_number_line(&chars);
    assert_eq!(magnitude(&test), 143);

    let test2 = create_number_line(&chars2);
    assert_eq!(magnitude(&test2), 3488)
}

fn main() {
    let mut numbers: Numbers = Numbers::new();
    read_file("puzzle.txt",parse_numbers , &mut numbers);

    // print_numbers(&numbers);
    // reduce_test();
    //magnitude_test();
    //println!(" divide test {:?}", divide_into_pair(10));
    let ans_1 = part_1(&numbers);
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2(&numbers);
    println!("Part 2: {}", ans_2);
}

