use std::env;
use std::fs;

const DAY: &str = "day2";


fn main() {
    println!("TEST");
    let in_files = vec![
        format!("/{}/src/{}_test.txt", DAY, DAY),
        format!("/{}/src/{}.txt", DAY, DAY)
    ];

    for path in in_files{
        let contents = read_lines(&path);
        let (cmds, amnts) = parse(&contents);
        part1(&cmds, &amnts);
        part2(&cmds, &amnts);
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

fn parse(contents: &str) -> (Vec<&str>, Vec<i32>){
    let mut cmds = Vec::new();
    let mut amnts: Vec<i32> = Vec::new();

    for s in contents.split("\n"){
        let mut iter = s.splitn(2, ' ');
        cmds.push(iter.next().unwrap());
        amnts.push(iter.next().unwrap().parse().unwrap());
    }
    return (cmds, amnts)
}

fn part1(cmds: &Vec<&str>, amnts: &Vec<i32>){

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for (&c, &a) in cmds.iter().zip(amnts.iter()) {
        match c {
            "forward" => { x = x + a; },
            "down" => { y = y + a; },
            "up" => { y = y - a; },
            _ => (),
        }
    }
    println!("part 1: {}", x * y)
}


fn part2(cmds: &Vec<&str>, amnts: &Vec<i32>){

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;
    for (&c, &a) in cmds.iter().zip(amnts.iter()) {
        match c {
            "forward" => { x = x + a; y = y + aim * a},
            "down" => { aim = aim + a},
            "up" => { aim = aim - a},
            _ => (),
        }
    
    
    }
    println!("part 2: {}", x * y);
}

