use std::env;
use std::fs;
use std::collections::VecDeque;

const DAY: &str = "day1";


fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let contents = read_lines(&test_file_path);
    part1(&contents);
    part2(&contents);

    println!("FOR REAL");
    let test_file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let contents = read_lines(&test_file_path);
    part1(&contents);
    part2(&contents);
}

fn read_lines(filename: &str) -> String{
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    
    println!("Reading {}", abs_filename);

    let contents = fs::read_to_string(abs_filename)
       .expect("Something went wrong reading the file");

    return contents
}

fn part1(contents: &str){
    let numbers: Vec<u16> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
    
    let mut deq = VecDeque::from(numbers);
    let mut cur = deq.pop_front().unwrap();

    let mut inc_cnt: u16 = 0;
    for num in deq.iter(){
        if *num > cur{
            inc_cnt = inc_cnt + 1;
        }
        cur = *num;
    }
    println!("part 1: {:?}", inc_cnt);
}


fn part2(contents: &str){
    let numbers: Vec<u16> = contents.split("\n").map(|x| x.parse().unwrap()).collect();

    let mut triplet_sums: Vec<u16> = Vec::new();
    let mut start=0;

    while start + 3 <= numbers.len(){
        let cur = &numbers[start..start+3];
        triplet_sums.push(cur.iter().sum());
        start += 1;
    } 

    let mut cur = triplet_sums[0];
    let mut inc_cnt = 0;
    for num in triplet_sums.iter(){
        if *num > cur{
            inc_cnt = inc_cnt + 1;
        }
        cur = *num;
    }
    println!("part 2: {:?}", inc_cnt);
}

