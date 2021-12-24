const TEST: &str = "3,4,3,1,2";
const REAL: &str = "2,5,2,3,5,3,5,5,4,2,1,5,5,5,5,1,2,5,1,1,1,1,1,5,5,1,5,4,3,3,1,2,4,2,4,5,4,5,5,5,4,4,1,3,5,1,2,2,4,2,1,1,2,1,1,4,2,1,2,1,2,1,3,3,3,5,1,1,1,3,4,4,1,3,1,5,5,1,5,3,1,5,2,2,2,2,1,1,1,1,3,3,3,1,4,3,5,3,5,5,1,4,4,2,5,1,5,5,4,5,5,1,5,4,4,1,3,4,1,2,3,2,5,1,3,1,5,5,2,2,2,1,3,3,1,1,1,4,2,5,1,2,4,4,2,5,1,1,3,5,4,2,1,2,5,4,1,5,5,2,4,3,5,2,4,1,4,3,5,5,3,1,5,1,3,5,1,1,1,4,2,4,4,1,1,1,1,1,3,4,5,2,3,4,5,1,4,1,2,3,4,2,1,4,4,2,1,5,3,4,1,1,2,2,1,5,5,2,5,1,4,4,2,1,3,1,5,5,1,4,2,2,1,1,1,5,1,3,4,1,3,3,5,3,5,5,3,1,4,4,1,1,1,3,3,2,3,1,1,1,5,4,2,5,3,5,4,4,5,2,3,2,5,2,1,1,1,2,1,5,3,5,1,4,1,2,1,5,3,5,2,1,3,1,2,4,5,3,4,3";

fn main() {
    let mut state: Vec<u32> = TEST.split(",").map(|x| x.parse().unwrap()).collect();
    //part1(&mut state)
    part2(&state)
}

fn part1(state: &mut Vec<u32>){
    // part 1
    let days = 80;
    let mut cur_day = 0;
    while cur_day < days{
        simulate(state);
        //println!("{:?}", state);
        cur_day += 1; 
    }
    println!("part 1: {}", state.len());
}


fn simulate(state: &mut Vec<u32>){
    let mut add_in_day = 0;
    for s in state.iter_mut(){
        match *s {
            | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => { *s = *s - 1;}
            | 0 => { *s = 6; add_in_day += 1 }
            _ => { println!("HO {}", *s)}
        }
    }
    if add_in_day > 0{
        let add = vec![8; add_in_day];
        state.extend(add);
    }
}


fn part2(state: &Vec<u32>){
    // part 1
    let mut counts: Vec<u32> = vec![0u32; 9];

    for c in state.iter(){
        counts[*c as usize] += 1;
    }
    
    let days = 18;
    let mut cur_day = 0;
    while cur_day < days{
        println!("{:?}", counts);
        let zero_cnts = counts[0];  
        //counts[5] += zero_cnts;
        counts.rotate_left(1);
        counts[8] = counts[0];
        
        cur_day += 1; 
    }

    let fish_sum: u32 = counts.iter().sum();
    println!("part 1: {}", fish_sum);
}