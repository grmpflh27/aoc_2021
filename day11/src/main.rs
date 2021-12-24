use ndarray::prelude::*;
use ndarray::{Array, Array2};
use std::env;
use std::fs;
use std::collections::HashSet;

const DAY: &str = "day11";


fn main() {
    let in_files = vec![
        //format!("/{}/src/{}_mini.txt", DAY, DAY),
        format!("/{}/src/{}_test.txt", DAY, DAY),
        //format!("/{}/src/{}.txt", DAY, DAY)
    ];

    for path in in_files{
        let contents = read_lines(&path);
        let mut octopusses = parse(&contents);
        part1(&mut octopusses);
        //part2(&heightmap);
    }
}

fn read_lines(filename: &str) -> String{
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);

    let contents = fs::read_to_string(abs_filename)
       .expect("Something went wrong reading the file");

    return contents
}

fn parse(contents: &str) -> Array2<u32>{
    let raw: Vec<&str> = contents.split("\n").collect();
    let (X, Y) = (raw[0].len(), raw.len());
    let mut octopusses: Array2<u32> = Array2::zeros((Y, X));
    
    for (i, line) in raw.iter().enumerate(){
        let array = Array::from_vec(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
        let mut row_slice = octopusses.slice_mut(s![i, ..]);
        row_slice += &ArrayView::from(&array);
    }
    
    return octopusses
}

fn part1(octopusses: &mut Array2<u32>){
    let epochs = 10;
    let mut cur_epoch = 0;

    let mut flash_cnt = 0;
    while cur_epoch < epochs{
        println!("{} out of {}", cur_epoch, epochs);
        flash_cnt += octo_flash(octopusses);
        cur_epoch += 1;
        println!("{:?}", octopusses);
    }
    println!("part 1: {}", flash_cnt);
}   


fn octo_flash(octopusses: &mut Array2<u32>) -> usize{
    let rows = octopusses.shape()[1];
    let cols = octopusses.shape()[0];
    let add_array: &Array2<u32> = &Array2::ones(octopusses.raw_dim());
    
    // increase by 1
    *octopusses += add_array;

    let mut flash_map: Array2<u32> = Array2::zeros(octopusses.raw_dim());
    let mut flash_idx: HashSet<usize> = HashSet::new();
   
    // find all > 9s
    let mut cnt = 0;
    loop {
        let cur_flash_idx = find_idx_greater_9(&octopusses);
        let cur_hashset: HashSet<_> = cur_flash_idx.iter().cloned().collect();
        
        if cur_flash_idx.is_empty(){
            break
        }
        
        if flash_idx.len() > 0 && flash_idx.is_disjoint(&cur_hashset){
            break
        }

        cnt += 1;
        if cnt == 15{
            panic!("this is a terrible mistake!");
        }
        for idx in cur_flash_idx.iter(){
            if flash_map[[idx % rows, idx / cols]] == 1{
                continue
            }
            let neighbor_idx = get_neighbor_idxs(octopusses, *idx);
            for neighbor in neighbor_idx.iter(){
                octopusses[[neighbor.1, neighbor.0]] += 1;
                println!("{:?}, {}", neighbor, octopusses[[neighbor.1, neighbor.0]])
            }
            octopusses[[idx % rows, idx / cols]] = 0;
            flash_map[[idx % rows, idx / cols]] = 1;
        }

         // set flashes 0
        for idx in cur_flash_idx.iter(){
            octopusses[[idx % rows, idx / cols]] = 0;
        }

        println!("{:?}", flash_map);
        flash_idx.extend(cur_flash_idx.iter());
    }

   
    return flash_idx.len();

}

fn find_idx_greater_9(octopusses: &Array2<u32>) -> Vec<usize>{
    let mut flash_idx: Vec<usize> = Vec::new();

    for (i, &val) in octopusses.iter().enumerate() {
        if val > 9{
            flash_idx.push(i);
        }
    }
    return flash_idx;
}

fn get_neighbor_idxs(octopusses: &Array2<u32>, flash_idx: usize) -> Vec<(usize, usize)>{
    let mut neighbor_idxs: Vec<(usize, usize)> = Vec::new();

    let rows = octopusses.shape()[0];
    let cols = octopusses.shape()[1];
    let col_idx = flash_idx % cols;
    let row_idx = flash_idx / rows;

    println!("{} ->  {} {}", flash_idx, row_idx, col_idx);

    // top left
    if row_idx != 0 && col_idx != 0{
        neighbor_idxs.push((row_idx - 1, col_idx - 1));
    }
    // top
    if row_idx != 0{
        neighbor_idxs.push((row_idx - 1, col_idx));
    }
    // top right
    if row_idx != 0 && col_idx != cols - 1{
        neighbor_idxs.push((row_idx - 1, col_idx + 1));
    }
    // left
    if col_idx != 0{
        neighbor_idxs.push((row_idx , col_idx - 1));
    }
    // right
    if col_idx != cols - 1{
        neighbor_idxs.push((row_idx , col_idx + 1));
    }
    // bottom left
    if row_idx != rows - 1 && col_idx != 0{
        neighbor_idxs.push((row_idx + 1, col_idx - 1));
    }
    // bottom
    if row_idx != rows - 1{
        neighbor_idxs.push((row_idx + 1, col_idx));
    }
    // bottom right
    if row_idx != rows - 1 && col_idx != cols - 1{
        neighbor_idxs.push((row_idx + 1, col_idx + 1));
    }
    return neighbor_idxs;
}
