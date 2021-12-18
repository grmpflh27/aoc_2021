use std::env;
use std::fs;
use std::collections::HashMap;

const DAY: &str = "day10";

fn main() {
    let in_files = vec![
        format!("/{}/src/{}_test.txt", DAY, DAY),
        format!("/{}/src/{}.txt", DAY, DAY)
    ];

    let open: Vec<char> = vec!['(', '[', '{', '<'];
    let close: Vec<char> = vec![')', ']', '}', '>'];
    

    for path in in_files{
        
        let contents = read_lines(&path);
        //part1(&contents, &open, &close);
        part2(&contents, &open, &close);
    }
}

fn read_lines(filename: &str) -> String{
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);

    let contents = fs::read_to_string(abs_filename)
       .expect("Something went wrong reading the file");

    return contents
}


fn part1(contents: &String, open: &Vec<char>, close: &Vec<char>){
    let mut syntax_error_score = 0;
    let syntax_error_score_map = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
        
    for line in contents.split("\n"){
        let mut chunk_stack: Vec<char> = Vec::new(); 
        for cur_char in line.chars(){
            match open.contains(&cur_char){
                true => chunk_stack.push(cur_char),
                false => {
                    let last = chunk_stack.pop().unwrap();
                    let close_idx = close.iter().position(|&x| x == cur_char).unwrap();
                    let open_idx = open.iter().position(|&x| x == last).unwrap();
                    if close_idx != open_idx{
                        syntax_error_score += syntax_error_score_map.get(&cur_char).unwrap();
                        break;
                    }
                }
            }
        }
    }
    println!("part 1: {}", syntax_error_score);
}


fn part2(contents: &String, open: &Vec<char>, close: &Vec<char>){
    let mut autocomplete_scores: Vec<u64> = Vec::new(); 
    let autocomplete_score_map = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);
        
    for line in contents.split("\n"){
        let mut is_corrupted = false;
        let mut chunk_stack: Vec<char> = Vec::new(); 
        for cur_char in line.chars(){
            match open.contains(&cur_char){
                true => chunk_stack.push(cur_char),
                false => {
                    let last = chunk_stack.pop().unwrap();
                    let close_idx = close.iter().position(|&x| x == cur_char).unwrap();
                    let open_idx = open.iter().position(|&x| x == last).unwrap();
                    if close_idx != open_idx{
                        is_corrupted = true;
                        break
                    }
                }
            }
        }

        if ! is_corrupted{
            // closing sequence score
            let mut cur_score: u64 = 0;
            while chunk_stack.len() > 0{
                let cur_char = chunk_stack.pop().unwrap();
                let open_idx = open.iter().position(|&x| x == cur_char).unwrap();
                cur_score = cur_score * 5 + autocomplete_score_map.get(&close[open_idx]).unwrap()
            }
            autocomplete_scores.push(cur_score)    
        }
    }

    // median
    autocomplete_scores.sort();
    let mid = autocomplete_scores.len() / 2;
    let median = autocomplete_scores[mid];

    println!("part 2: {}", median);
}