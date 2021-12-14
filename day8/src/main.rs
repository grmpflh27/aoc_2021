use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

const DAY: &str = "day8";


fn main() {
    let in_files = vec![
        //format!("/{}/src/{}_test_mini.txt", DAY, DAY),
        //format!("/{}/src/{}_test.txt", DAY, DAY),
        format!("/{}/src/{}.txt", DAY, DAY)
    ];

    for path in in_files{
        let contents = read_lines(&path);
        //part1(&contents);
        part2(&contents);
    }
}

fn read_lines(filename: &str) -> String{
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);

    let contents = fs::read_to_string(abs_filename)
       .expect("Something went wrong reading the file");

    return contents
}

fn part1(contents: &str){
    let mut cnt = 0;
    for s in contents.split("\n"){
        let parts: Vec<&str> = s.splitn(2, " | ").collect();
        
        for cur in parts[1].split(" "){
            match cur.len(){
                | 2 | 3 | 4 | 7  => {
                    cnt += 1;
                }
                _ => ()
            }
        }
    }
    println!("part1: {}", cnt);
}


fn part2(contents: &str){

    let base: i32 = 10;
    let mut part2_sum = 0;
    for s in contents.split("\n"){
        let parts: Vec<&str> = s.splitn(2, " | ").collect();
        let signal_pattern: Vec<&str> = parts[0].split(" ").collect();
        let out_value: Vec<String> = parts[1].split(" ").map(|x| x.to_string()).collect();
        // sort everything desc
        let mut charred: Vec<Vec<char>> = signal_pattern.iter().map(|x| x.chars().collect()).collect();
        
        let mut sorted: Vec<String> = Vec::new();
        for c in charred.iter_mut(){
            c.sort_by(|a, b| b.cmp(a));
            sorted.push(c.iter().collect());
        }

        let map = solve(&signal_pattern);

        let mut cur_sum: i32 = 0;
        let mut exp: i32 = 3;
        for val in out_value.iter(){
            let mut chars: Vec<char> = val.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            let val_sorted: String = chars.iter().collect();
            // get index
            let idx: usize = sorted.iter().position(|x| *x == val_sorted).unwrap();
            let cur_num: i32 = *map.get(&idx).unwrap();

            cur_sum += cur_num * base.pow(exp as u32);
            exp -= 1;
        }
        part2_sum += cur_sum

    }
    println!("part2: {}", part2_sum);
}


//  aaaa
// b    c
// b    c
//  dddd
// e    f
// e    f
//  gggg


fn solve(signal_pattern: &Vec<&str>) -> HashMap<usize, i32>{
    let mut da_map: HashMap<usize, i32> = HashMap::new();

    let one: String = signal_pattern.iter().find(|&&x| x.len() == 2).unwrap().to_string();
    let one_idx: usize = signal_pattern.iter().position(|&x| x == one).unwrap();
    da_map.insert(one_idx, 1);
    let seven: String = signal_pattern.iter().find(|&x| x.len() == 3).unwrap().to_string();
    let seven_idx: usize = signal_pattern.iter().position(|&x| x == seven).unwrap();
    da_map.insert(seven_idx, 7);
    let eight: String = signal_pattern.iter().find(|&x| x.len() == 7).unwrap().to_string();
    let eight_idx: usize = signal_pattern.iter().position(|&x| x == eight).unwrap();
    da_map.insert(eight_idx, 8);
    let four: String = signal_pattern.iter().find(|&x| x.len() == 4).unwrap().to_string();
    let four_idx: usize = signal_pattern.iter().position(|&x| x == four).unwrap();
    da_map.insert(four_idx, 4);

    // only 2 has segment f missing --> others fully intersect with 1
    let len_5s: Vec<String> = signal_pattern.iter().filter(|&x| x.len() == 5).map(|x| x.to_string()).collect();
    let one_chars: Vec<char> = one.chars().collect();

    let mut diffs_5s: HashMap<String, String> = HashMap::new();
    for word in len_5s.iter(){
        let cur_chars: Vec<char> = word.chars().collect();
        let diff = get_diff_char(&one_chars, &cur_chars);
        if diff.len() == 0{
            let three_idx: usize = signal_pattern.iter().position(|&x| x == word).unwrap();
            da_map.insert(three_idx, 3);
        } else {
            diffs_5s.insert(String::from(word), diff[0].to_string());
        }
    }

    // 2 and 5 -> only has a diff that nobody else has
    for (val, diff_char) in &diffs_5s {
        let others: Vec<&&str> = signal_pattern.iter().filter(|x| *x != val).collect();
        let has_segment_f = others.iter().all(|&x| x.contains(diff_char));
        let idx: usize = signal_pattern.iter().position(|&x| x == val).unwrap();
        if has_segment_f{
            da_map.insert(idx, 2);
        } else {
            da_map.insert(idx, 5);
        }
    }

    // only 0, 6 and 9 missing at this point -> 6 is not fully covered by 1
    let len_6s: Vec<String> = signal_pattern.iter().filter(|&x| x.len() == 6).map(|x| x.to_string()).collect();
    let mut zero_or_nine: Vec<String> = Vec::new(); 
    for word in len_6s.iter(){
        let cur_chars: Vec<char> = word.chars().collect();
        let diff = get_diff_char(&one_chars, &cur_chars);
        if diff.len() != 0{
            let six_idx: usize = signal_pattern.iter().position(|&x| x == word).unwrap();
            da_map.insert(six_idx, 6);
        } else {
            zero_or_nine.push(word.to_string());
        }
    }

    // only 9 intersects with 4
    let four_chars: Vec<char> = four.chars().collect();
    for word in zero_or_nine.iter(){
        let cur_chars: Vec<char> = word.chars().collect();
        let diff = get_diff_char(&four_chars, &cur_chars);
        let idx: usize = signal_pattern.iter().position(|&x| x == word).unwrap();
        if diff.len() == 0{
            da_map.insert(idx, 9);
        } else {
            da_map.insert(idx, 0);
        }
    }

    return da_map;
}


fn get_diff_char(bigger: &Vec<char>, smaller: &Vec<char>) -> Vec<char>{
    let hashset_bigger: HashSet<_> = bigger.iter().cloned().collect();
    let hashset_smaller: HashSet<_> = smaller.iter().cloned().collect();
    let diff = &hashset_bigger - &hashset_smaller;
    return diff.iter().cloned().collect()
}