extern crate file_reader;
use file_reader::read_file;

type BitStr = Vec<Vec<char>>;
type BITS = Vec<char>;

fn parse_bits(s: &str, bit_str: &mut BitStr) {
    let row = s.chars().collect::<Vec<char>>();
    bit_str.push(row)
}

fn decode_bits(bits: &BITS) -> BITS {
    let mut decoded: BITS = BITS::new();

    for b in bits {
        match b {
            '0' => decoded.append(&mut vec!['0', '0', '0', '0']), // 0 = 0000
            '1' => decoded.append(&mut vec!['0', '0', '0', '1']), // 1 = 0001
            '2' => decoded.append(&mut vec!['0', '0', '1', '0']), // 2 = 0010
            '3' => decoded.append(&mut vec!['0', '0', '1', '1']), // 3 = 0011
            '4' => decoded.append(&mut vec!['0', '1', '0', '0']), // 4 = 0100
            '5' => decoded.append(&mut vec!['0', '1', '0', '1']), // 5 = 0101
            '6' => decoded.append(&mut vec!['0', '1', '1', '0']), // 6 = 0110
            '7' => decoded.append(&mut vec!['0', '1', '1', '1']), // 7 = 0111
            '8' => decoded.append(&mut vec!['1', '0', '0', '0']), // 8 = 1000
            '9' => decoded.append(&mut vec!['1', '0', '0', '1']), // 9 = 1001
            'A' => decoded.append(&mut vec!['1', '0', '1', '0']), // A = 1010
            'B' => decoded.append(&mut vec!['1', '0', '1', '1']), // B = 1011
            'C' => decoded.append(&mut vec!['1', '1', '0', '0']), // C = 1100
            'D' => decoded.append(&mut vec!['1', '1', '0', '1']), // D = 1101
            'E' => decoded.append(&mut vec!['1', '1', '1', '0']), // E = 1110
            'F' => decoded.append(&mut vec!['1', '1', '1', '1']), // F = 1111
            _ => panic!("{}", "Invalid char"),
        }
    }

    decoded
}

fn bits_to_string(bits: &BITS, start: usize, len: usize) -> String {
    let mut s = "".to_string();

    for i in start..len + start {
        s.push(bits[i])
    }
    s
}

fn binary_to_int(s: &str) -> i64 {
    i64::from_str_radix(&s, 2).unwrap()
}

fn bits_to_int(bits: &BITS, start: usize, length: usize) -> i64 {
    binary_to_int(&bits_to_string(&bits, start, length))
}

fn read_packet(bits: &BITS, start: usize) -> (i64, i64) {   
    let mut version_sum = 0;
    let mut sum_totals = 0;
    
    let version = binary_to_int(&bits_to_string(&bits, start, 3));
    version_sum += version;
    let p_type = binary_to_int(&bits_to_string(&bits, start + 3, 3));

    if p_type == 4 {
         let (_, _) = read_literal(&bits, start + 6);     
    } else {
       let (_, v_sum, val) = read_operator(&bits, start + 6, p_type);
       
       version_sum += v_sum;
       sum_totals = val     
    }

    (version_sum, sum_totals)
}

fn read_number_of_packet(bits: &BITS, start: usize, stop_at: usize) -> (usize, i64, Vec<i64>) {
    let mut numb = start;   
    let mut version_sum = 0;
    let mut values: Vec<i64> = Vec::new();
   for _ in 0..stop_at {
    let version = binary_to_int(&bits_to_string(&bits, numb, 3));
    version_sum += version;
    let p_type = binary_to_int(&bits_to_string(&bits, numb + 3, 3));

    if p_type == 4 {
        let (nm, val) = read_literal(&bits, numb + 6);
        numb = nm;
        values.push(val);      
    } else {
       let (n, v_sum, val) = read_operator(&bits, numb + 6, p_type);
       numb = n;
       version_sum += v_sum;
       values.push(val);
    }   
   }  

    (numb, version_sum, values)
}


fn read_packets_until(bits: &BITS, start: usize, length: usize) -> (usize, i64, Vec<i64>) {
    let mut values: Vec<i64> = Vec::new();
    let mut numb = start;   
    let mut version_sum = 0;
    
   loop {       
    let version = binary_to_int(&bits_to_string(&bits, numb, 3));
    version_sum += version;
    let p_type = binary_to_int(&bits_to_string(&bits, numb + 3, 3));   

    if p_type == 4 {
         let (nm, val) = read_literal(&bits, numb + 6);
         numb = nm;
        values.push(val)       
    } else {
       let (n, v_sum, val) = read_operator(&bits, numb + 6, p_type);
       numb = n;
       version_sum += v_sum;
       values.push(val);
      
    }   

    if numb == start + length {
        break;
    }
   }  

    (numb, version_sum, values)
}

fn read_literal(bits: &BITS, start: usize) -> (usize, i64) {
    let mut s = "".to_string();
    let mut i = start;
    loop {
        let b = bits_to_string(&bits, i + 1, 4);
        s += &b;      
       
        if bits[i] == '0' {
            break;
        }
        i += 5;
    }
   (i + 5, binary_to_int(&s))
}

fn read_operator(bits: &BITS, start: usize, p_type: i64) -> (usize, i64, i64) {        
    if bits[start] == '0' {
        let length = bits_to_int(&bits, start + 1, 15);
        let sub_start = start + 1 + 15;
        let (numb, version_sum, values) = read_packets_until(&bits, sub_start, length as usize );
        let new_val = process_numbs(p_type, &values);
        return(numb, version_sum, new_val) ;
        
    } else {
        let n = bits_to_int(&bits, start + 1, 11);
        let sub_start = start + 1 + 11;
        let (numb, version_sum, values) = read_number_of_packet(&bits, sub_start, n as usize);  
        let new_val = process_numbs(p_type, &values);
        return(numb, version_sum, new_val) ;      
    }      
}

fn process_numbs(p_type: i64, numbs: &Vec<i64>) -> i64 {
    let mut tot = 0;

        match p_type {
            0 => {
                for n in numbs {
                    tot += n;
                }
            },
            1 => {
                tot = 1;
                for n in numbs {
                    tot *= n;
                }
            },
            2 => {
               tot = i64::MAX;
                for n in numbs {
                    if n < &tot {
                        tot = *n;
                    }
                }

            },
            3 => {
                tot = i64::MIN;
                for n in numbs {
                    if n > &tot {
                        tot = *n;
                    }
                }
            },            
            5 => {
                if numbs[0] > numbs[1] {
                    tot = 1;
                } else {
                    tot = 0;
                }

            },
            6 => {
                if numbs[0] < numbs[1] {
                    tot = 1;
                } else {
                    tot = 0;
                }
            },
            7 => {
                if numbs[0] == numbs[1] {
                    tot = 1;
                } else {
                    tot = 0;
                }
            },
            _ => panic!("{}","INvalid Type")
        }
    tot
}

fn part_1_and_2(bits: &BITS) -> (i64,i64) {
    let decoded = decode_bits(bits);
    read_packet(&decoded, 0)
}

fn main() {
    let mut bit_str: BitStr = BitStr::new();
    read_file("puzzle.txt", parse_bits, &mut bit_str);

    for bits in bit_str {
       let (ans_1, ans_2) = part_1_and_2(&bits);
        println!("Part 1: {}", ans_1);
        println!("Part 2: {}", ans_2);
        println!("-----------------");
    }     
}
