use std::env;
use std::fs;
use std::collections::HashSet;
use ndarray::{Array, Array2};

const DAY: &str = "day13";

#[derive(Debug, PartialEq, Eq, Hash)]
struct Dot(u32, u32);

#[derive(Debug)]
struct FoldingInstruction{
    value: u32,
    dim: u32,
}

fn main() {
    let in_files = vec![
        //format!("/{}/src/{}_test.txt", DAY, DAY),
        format!("/{}/src/{}.txt", DAY, DAY)
    ];

    for path in in_files{
        let contents = read_lines(&path);
        let (mut dots, folding_instr) = parse(&contents);
        part1(&mut dots, &folding_instr);
        part2(&dots);
    }
}

fn read_lines(filename: &str) -> String{
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);

    let contents = fs::read_to_string(abs_filename)
       .expect("Something went wrong reading the file");

    return contents
}

fn parse(contents: &str) -> (Vec<Dot>, Vec<FoldingInstruction>){
    let mut dots: Vec<Dot> = Vec::new();
    let sections: Vec<&str> = contents.split("\n\n").collect();
    let dot_coord_lines: Vec<&str> = sections[0].split("\n").collect();
    
    for line in dot_coord_lines.iter(){
        let parts: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        dots.push(Dot(parts[0], parts[1]));
    }

    let mut folding_instr: Vec<FoldingInstruction> = Vec::new();

    let folding_lines: Vec<&str> = sections[1].split("\n").collect();
    for line in folding_lines.iter(){
        let parts: Vec<&str> = line.split("=").collect();

        folding_instr.push(FoldingInstruction{
            value: parts[1].parse().unwrap(),
            dim: match parts[0]{
                "y" => 1,
                "x" => 0,
                _ => 99,
            }
        })
    }    
    return (dots, folding_instr)
}

fn part1(dots: &mut Vec<Dot>, folding_instr: &Vec<FoldingInstruction>){
    println!("{:?}", folding_instr);

    for (i, instr) in folding_instr.iter().enumerate(){
        if i == 1{
            // drop duplicates
            let hs: HashSet<&Dot> = dots.iter().collect();
            println!("part1: {:?}", hs.len());
        }

        for dot in dots.iter_mut(){
            if instr.dim == 1{
                if dot.1 < instr.value{
                    continue
                }
                let delta: u32 = dot.1 - instr.value;
                dot.1 = instr.value - delta;
            } else {
                if dot.0 < instr.value{
                    continue
                }
                let delta: u32 = dot.0 - instr.value;
                dot.0 = instr.value - delta;
            }
        }
    }
}

fn part2(dots: &Vec<Dot>){
    let hs: HashSet<&Dot> = dots.iter().collect();
    println!("part2: {:?}", hs.len());

    let (mut X, mut Y) = (0, 0);
    // get max dimensions
    for cur in &hs{
        if cur.0 > X{
            X = cur.0;
        }
        if cur.1 > Y{
            Y = cur.1;
        }
    }

    let mut map: Array2<char> = Array::from_elem((Y as usize + 1, X as usize + 1), ' ');

    for cur in &hs{
        map[[cur.1 as usize, cur.0 as usize ]] = '█';
    }

    println!("{:?}", map);
}