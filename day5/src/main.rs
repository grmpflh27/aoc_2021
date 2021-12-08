use std::env;
use std::fs;
use std::cmp;


const DAY: &str = "day5";


fn main() {
    let in_files = vec![
        format!("/{}/src/{}_test.txt", DAY, DAY),
        format!("/{}/src/{}.txt", DAY, DAY)
    ];

    for path in in_files{
        let contents = read_lines(&path);
        let (lines, dims) = parse(&contents);

        part1(&lines, dims);
        part2(&lines, dims);
    }
}

fn read_lines(filename: &str) -> String{
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    let contents = fs::read_to_string(abs_filename)
       .expect("Something went wrong reading the file");

    return contents
}

#[derive(Debug, Clone, PartialEq)]
struct Point(i32, i32);

#[derive(Debug, Clone)]
struct Line{
    start: Point,
    end: Point
}

impl Line{
    fn is_horizontal(&self) -> bool{
        self.start.1 == self.end.1
    }
    
    fn is_vertical(&self) -> bool{
        self.start.0 == self.end.0
    }

    fn to_points(&self) -> Vec<Point>{
        let mut pts: Vec<Point> = Vec::new();
        pts.push(Point(self.start.0, self.start.1));

        let (mut cur_x, mut cur_y) = (self.start.0, self.start.1);
        let (mut x_dir, mut y_dir) = (1, 1);
        
        if self.start.0 > self.end.0{
            x_dir = -1    
        }
        if self.start.1 > self.end.1{
            y_dir = -1    
        }

        if self.is_vertical(){
            loop {
                cur_y = cur_y + y_dir;
                pts.push(Point(cur_x, cur_y));

                if cur_y == self.end.1{
                    break
                }
            }
        } else if self.is_horizontal(){
            loop {
                cur_x = cur_x + x_dir;
                pts.push(Point(cur_x, cur_y));
                
                if cur_x == self.end.0{
                    break
                }
            }
        } else {
            // diagonal
            loop {
                cur_x = cur_x + x_dir;
                cur_y = cur_y + y_dir;
                pts.push(Point(cur_x, cur_y));

                if cur_x == self.end.0 || cur_y == self.end.1{
                    break
                }
            }
        }

        return pts
    }
}


fn parse(contents: &str) -> (Vec<Line>, (i32, i32)) {
    let mut lines: Vec<Line> = Vec::new();

    let _in: Vec<&str> = contents.split("\n").collect();

    let mut dim_min: i32 = 10000;
    let mut dim_max: i32 = 0;

    for line in _in{
        let parts: Vec<&str> = line.split(" -> ").collect();
    
        let from: Vec<i32> = parts[0].split(",").map(|x| x.parse().unwrap()).collect();
        let to: Vec<i32> = parts[1].split(",").map(|x| x.parse().unwrap()).collect();

        //ballpark
        dim_min = cmp::min(cmp::min(from[0], from[1]), dim_min);
        dim_max = cmp::max(cmp::max(to[0], to[1]), dim_max);
        
        lines.push(Line{
            start: Point(from[0], from[1]),
            end: Point(to[0], to[1])
        });
    }
    println!("{}, {}", dim_min, dim_max);
    return (lines, (dim_min, dim_max))
}

fn part1(lines: &Vec<Line>, dims: (i32, i32)){
    // filter diagonal
    let mut non_diagonal: Vec<&Line> = Vec::new();
    for l in lines.iter(){
        if l.is_horizontal() || l.is_vertical(){
            non_diagonal.push(l);
        }
    }
    println!("{} non diagonal lines", non_diagonal.len());

    let mut all_points: Vec<Point> = Vec::new();
    for n in non_diagonal.iter(){
        all_points.extend(n.to_points());
    }
    println!("{} total points", all_points.len());

    // count at least 2
    let mut cnt_overlap_gte_2 = 0;
    
    // super slow - better would be to iterate over board
    //let mut checked: Vec<&Point> = Vec::new();
    // for pt in all_points.iter(){
    //     if checked.contains(&pt){
    //         continue
    //     }

    //     if all_points.iter().filter(|n| *n == pt).count() >= 2{
    //         cnt_overlap_gte_2 = cnt_overlap_gte_2 + 1;
    //     }
    //     checked.push(pt);
    // }
    
    // assumption: square board - ~100k ops -> ~10k ops
    let mut cnt = 1;
    for x in dims.0..dims.1+1{
        for y in dims.0..dims.1+1{
            if all_points.iter().filter(|n| n.0 == x && n.1 == y).count() >= 2{
               cnt_overlap_gte_2 = cnt_overlap_gte_2 + 1;
            }
            cnt = cnt + 1;
        }
    }

    
    println!("part 1: {}", cnt_overlap_gte_2)
}


fn part2(lines: &Vec<Line>, dims: (i32, i32)){

    let mut all_points: Vec<Point> = Vec::new();
    for n in lines.iter(){
        all_points.extend(n.to_points());
    }
    println!("{} total points", all_points.len());
    
    let mut cnt_overlap_gte_2 = 0;
    
    // assumption: square board
    let mut cnt = 1;
    for x in dims.0..dims.1+1{
        for y in dims.0..dims.1+1{
            if all_points.iter().filter(|n| n.0 == x && n.1 == y).count() >= 2{
               cnt_overlap_gte_2 = cnt_overlap_gte_2 + 1;
            }
            cnt = cnt + 1;
        }
    }

    println!("part 2: {}", cnt_overlap_gte_2)
}

