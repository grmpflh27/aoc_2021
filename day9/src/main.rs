use std::env;
use std::fs;
use std::collections::HashMap;

const DAY: &str = "day9";


fn main() {
    let in_files = vec![
        //format!("/{}/src/{}_test.txt", DAY, DAY),
        format!("/{}/src/{}.txt", DAY, DAY)
    ];

    for path in in_files{
        let contents = read_lines(&path);
        let heightmap = parse(&contents);
        //part1(&heightmap);
        part2(&heightmap);
    }
}

fn read_lines(filename: &str) -> String{
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);

    let contents = fs::read_to_string(abs_filename)
       .expect("Something went wrong reading the file");

    return contents
}

fn parse(contents: &str) -> Vec<Vec<u32>>{
    let mut heightmap: Vec<Vec<u32>> = Vec::new();
    for line in contents.split("\n"){
        let heights: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        heightmap.push(heights);
    }
    return heightmap
}

fn get_neighbors(heightmap: &Vec<Vec<u32>>, row_idx: usize, col_idx: usize) -> (Vec<u32>, Vec<(usize, usize)>){
    let (X, Y) = (heightmap[0].len(), heightmap.len());
    let mut neighbor_vals: Vec<u32> = Vec::new();
    let mut neighbor_idxs: Vec<(usize, usize)> = Vec::new();

    // top
    if row_idx != 0{
        neighbor_vals.push(heightmap[row_idx - 1][col_idx]);
        neighbor_idxs.push((row_idx - 1, col_idx));
    }
    // bottom
    if row_idx != Y - 1{
        neighbor_vals.push(heightmap[row_idx + 1][col_idx]);
        neighbor_idxs.push((row_idx + 1, col_idx));
    }
    // left
    if col_idx != 0{
        neighbor_vals.push(heightmap[row_idx][col_idx - 1]);
        neighbor_idxs.push((row_idx , col_idx - 1));
    }
    // right
    if col_idx != X - 1{
        neighbor_vals.push(heightmap[row_idx][col_idx + 1]);
        neighbor_idxs.push((row_idx , col_idx + 1));
    }

    return (neighbor_vals, neighbor_idxs)
}

fn part1(heightmap: &Vec<Vec<u32>>){
    //iterate
    let mut risk_level_sum = 0;
    for (row_idx, line) in heightmap.iter().enumerate(){
        for (col_idx, height) in line.iter().enumerate(){
            let (neighbors, _) = get_neighbors(&heightmap, row_idx, col_idx);

            let all_bigger = neighbors.iter().all(|x| x > height);
            if all_bigger{
                risk_level_sum += height + 1;
            }
        }
    }
    println!("part 1: {}", risk_level_sum);
}

fn part2(heightmap: &Vec<Vec<u32>>){
    let (X, Y) = (heightmap[0].len(), heightmap.len());

    let mut basin_map: Vec<Vec<u16>> = vec![vec![0; X]; Y];
    let mut counts = HashMap::new();
    
    let mut basin_id_cnt: u16 = 1;
    let mut cur_basin_id;
    let mut to_merge = HashMap::new();
    for (row_idx, line) in heightmap.iter().enumerate(){
        for (col_idx, height) in line.iter().enumerate(){
            if *height == 9{
                continue
            }

            let (neighbor_vals, neighbor_idxs) = get_neighbors(&heightmap, row_idx, col_idx);
            let mut neighbor_basin_ids: Vec<u16> = neighbor_idxs.iter().map(|x| basin_map[x.0][x.1]).filter(|x| *x != 0).collect();

            if neighbor_basin_ids.is_empty(){
                cur_basin_id = basin_id_cnt;
                basin_id_cnt += 1;
                counts.insert(cur_basin_id, 1);
            } else {
                // asumption: all the same not given ... 
                neighbor_basin_ids.sort_unstable();
                neighbor_basin_ids.dedup();

                // ... but merge basins together lat0r
                if neighbor_basin_ids.len() > 1{
                    to_merge.insert(neighbor_basin_ids[0], neighbor_basin_ids[1]);
                }
                cur_basin_id = neighbor_basin_ids[0];

                let old_count = counts.get(&cur_basin_id).unwrap();
                counts.insert(cur_basin_id, old_count + 1);
            }

            if basin_map[row_idx][col_idx] == 0{
                basin_map[row_idx][col_idx] = cur_basin_id;
            }
            
            
            for (&v, &idxs) in neighbor_vals.iter().zip(neighbor_idxs.iter()){
                match v{
                    | 9  => (),
                    _ => {
                        basin_map[idxs.0][idxs.1] = cur_basin_id;
                    },
                }
            }
        }
    }

    // merge
    let (X, Y) = (basin_map[0].len(), basin_map.len());
    for (from, to) in &to_merge {
        for row_idx in 0..Y{
            for col_idx in 0..X{
                if basin_map[row_idx][col_idx] == *from{
                    basin_map[row_idx][col_idx] = *to;
                }
            }
        }
    }


    let mut vec: Vec<u16> = counts.into_values().collect();
    vec.sort();
    vec.reverse();
    let top3 = vec[0..3].to_vec();

    let part2: u32 = top3[0] as u32 * top3[1] as u32 * top3[2] as u32;
    println!("part 2: {}", part2);
}