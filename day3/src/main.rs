use std::env;
use std::fs;

const DAY: &str = "day3";


fn main() {
    println!("TEST");
    let in_files = vec![
        format!("/{}/src/{}_test.txt", DAY, DAY),
        format!("/{}/src/{}.txt", DAY, DAY)
    ];

    for path in in_files{
        let contents = read_lines(&path);
        let vec: Vec<&str> = contents.split("\n").collect();
        part1(&vec);
        part2(&vec);
    }
}

fn read_lines(filename: &str) -> String{
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    
    println!("Reading {}", abs_filename);

    let contents = fs::read_to_string(abs_filename)
       .expect("Something went wrong reading the file");

    return contents
}

fn part1(vec: &Vec<&str>){
    let num_cols = vec[0].len();
    let thresh = vec.len() / 2;
    let mut cnts = vec![0; num_cols];

    for s in vec.iter(){
        for (i, symbol) in s.chars().enumerate(){
            match symbol{
                '1' => {
                    cnts[i] = cnts[i] + 1;
                },
                _ => ()
            }
        }
    }

    let bin_map: Vec<_> = cnts.iter().map(|&x| {
        if x >  thresh{
            '1'
        } else {
            '0'
        }
    }).collect();

    let bin_str: String = bin_map.iter().collect();
    let t1 = isize::from_str_radix(&bin_str, 2).unwrap();

    // ugly AF   
    let bin_map2: Vec<_> = cnts.iter().map(|&x| {
        if x > thresh{
            '0'
        } else {
            '1'
        }
    }).collect();

    let bin_str2: String = bin_map2.iter().collect();
    let t2 = isize::from_str_radix(&bin_str2, 2).unwrap();

    println!("part 1: {}", t1 * t2);
}


fn part2(vec: &Vec<&str>){
    let mut copy = vec.to_vec();

    // oxygen generator rating
    let mut char_idx = 0;
    loop{
        let cur_msb = get_msb(&copy, char_idx);
        let mut _tmp: Vec<&str> = Vec::new();
        for c in copy{
            let cur_char = c.chars().nth(char_idx).unwrap();
            if cur_char == cur_msb{
                _tmp.push(c);
            }
        }

        copy = _tmp;        
        if copy.len() == 1{
            break
        }
        char_idx = char_idx + 1;
    }
    
    let oxygen_gen_rating = isize::from_str_radix(&copy[0], 2).unwrap();

    // C02 scrubber rating
    copy = vec.to_vec();
    let mut char_idx = 0;
    loop{
        let cur_lsb = get_lsb(&copy, char_idx);
        
        let mut _tmp: Vec<&str> = Vec::new();
        for c in copy{
            let cur_char = c.chars().nth(char_idx).unwrap();
            if cur_char == cur_lsb{
                _tmp.push(c);
            }
        }

        copy = _tmp;        
        if copy.len() == 1{
            break
        }
        char_idx = char_idx + 1;
    }

    let c02_scrub_rating = isize::from_str_radix(&copy[0], 2).unwrap();
    
    println!("part 2: {}", oxygen_gen_rating * c02_scrub_rating);
}


fn get_msb(vec: &Vec<&str>, idx: usize) -> char{
    let mut zero_cnt = 0;
    let mut one_cnt = 0;
    for line in vec{  
        match line.chars().nth(idx).unwrap(){
            '0' => {
                zero_cnt = zero_cnt + 1
            },
            '1' => {
                one_cnt = one_cnt + 1
            },
            _ => ()
        }
    }

    if zero_cnt > one_cnt{
        return '0'
    }
    return '1'
}

fn get_lsb(vec: &Vec<&str>, idx: usize) -> char{
    let mut zero_cnt = 0;
    let mut one_cnt = 0;
    for line in vec{  
        match line.chars().nth(idx).unwrap(){
            '0' => {
                zero_cnt = zero_cnt + 1
            },
            '1' => {
                one_cnt = one_cnt + 1
            },
            _ => ()
        }
    }

    if zero_cnt > one_cnt{
        return '1'
    }
    return '0'
}
