use std::env;
use std::fs;
use regex::Regex;
use std::collections::VecDeque;

const DAY: &str = "day4";


fn main() {
    let in_files = vec![
        format!("/{}/src/{}_test.txt", DAY, DAY),
        format!("/{}/src/{}.txt", DAY, DAY)
    ];

    for path in in_files{
        let contents = read_lines(&path);
        let game = parse(&contents);
        part1(game);

        let game2 = parse(&contents);
        part2(game2);
    }
}

fn read_lines(filename: &str) -> String{
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    let contents = fs::read_to_string(abs_filename)
       .expect("Something went wrong reading the file");

    return contents
}

#[derive(Debug, Clone)]
struct Board {
    id: usize,
    numbers: Vec<i32>,
    opts: Vec<Vec<i32>>,
}

struct Game {
    draws: VecDeque<i32>,
    drawn: Vec<i32>,
    boards: Vec<Board>,
    winner_idx: Option<usize>,
}

#[derive(Eq, PartialEq)]
enum GameMode {
    FirstWins,
    LastWins,
}


impl Game {
    fn draw(&mut self) {
        let drawn = self.draws.pop_front().unwrap();
        self.drawn.push(drawn);
    }

    pub fn play_round(&mut self, mode: GameMode) -> bool{
        self.draw();

        if mode == GameMode::FirstWins{
            for n in 0..self.boards.len(){
                let res = self.boards[n].is_winner(&self.drawn);
                
                if res {
                    self.winner_idx = Some(self.boards[n].id);
                    return true
                }
            }
        } else if mode == GameMode::LastWins {
            let mut to_keep: Vec<Board> = Vec::new();
            for n in 0..self.boards.len(){
                let res = self.boards[n].is_winner(&self.drawn);
                
                if ! res {
                    to_keep.push(self.boards[n].clone());
                }
            }
            self.boards = to_keep;

            if self.boards.len() == 1{
                return true
            }
        }

        return false
    }
}

impl Board {
    fn is_winner(&mut self, drawn: &Vec<i32>) -> bool{
        let mut matches = 0;
        for cur_opt in self.opts.iter(){
            for draw in drawn.iter(){
                if cur_opt.contains(&draw){
                    matches = matches + 1
                }
                if matches == 5{
                    return true
                }
            }
            matches = 0;
        }
        return false;
    }

    fn get_unmarked_numbers(&self, drawn: Vec<i32>) -> Vec<i32>{
        let mut difference = vec![];
        for n in self.numbers.iter() {
            if !drawn.contains(&n) {
                difference.push(*n);
            }
        }
        return difference
    }
}


fn parse(contents: &str) -> Game {
    let mut lines: Vec<&str> = contents.split("\n").collect();
    let draws: Vec<i32> = lines[0].split(",").map(|x| x.parse().unwrap()).collect();
    let draws = VecDeque::from(draws);
    
    lines.drain(0..1);
    let mut boards: Vec<Board> = Vec::new();

    let mut board_id = 0;
    while lines.len() > 0{
        let board_lines = lines[1..6].to_owned();
        boards.push(parse_board(board_lines, board_id));
        lines.drain(0..6);
        board_id = board_id + 1;
    }

    return Game{
        draws: draws,
        drawn: Vec::new(),
        boards: boards,
        winner_idx: None,
    }
}

fn parse_board(board_lines: Vec<&str>, board_id: u8) -> Board{
    let re = Regex::new(r"\s+").unwrap();
    
    let mut opts: Vec<Vec<i32>> = Vec::new();
    for line in board_lines{
        let line = re.replace_all(line, " ");
        let cur_row: Vec<i32> = line.trim().split(" ").map(|x| x.parse().unwrap()).collect();
        opts.push(cur_row);
    }

    let numbers = opts.to_vec().iter().flat_map(|arr| arr.iter()).cloned().collect();

    // append col opts
    let mut idx = 0;
    
    let mut col_opts: Vec<Vec<i32>> = Vec::new();
    while idx < 5{
        let mut cur_col_opt: Vec<i32> = Vec::new();
        for opt in opts.iter(){
            cur_col_opt.push(opt[idx])
        }
        col_opts.push(cur_col_opt);
        idx = idx + 1
    }
    opts.extend(col_opts);

    return Board {
        id: board_id as usize,
        numbers, 
        opts,
    }
}

fn part1(mut game: Game){
    let mut finished = false;
    while !finished {
        finished = game.play_round(GameMode::FirstWins);
    }

    let last_drawn = game.drawn[game.drawn.len() - 1] as i32;
    let winner = game.winner_idx.unwrap();
    let board = &game.boards[winner];

    let unmarked = board.get_unmarked_numbers(game.drawn);
    let sum: i32 = unmarked.iter().sum();
    println!("part 1: {}", last_drawn * sum)
}


fn part2(mut game: Game){
    let mut finished = false;
    while !finished {
        finished = game.play_round(GameMode::LastWins);
    }

    // one left - finish it
    finished = false;
    while !finished {
        finished = game.play_round(GameMode::FirstWins);
    }

    let last_drawn = game.drawn[game.drawn.len() - 1] as i32;
    let board = &game.boards[0];

    let unmarked = board.get_unmarked_numbers(game.drawn);
    let sum: i32 = unmarked.iter().sum();
    println!("part 2: {}", last_drawn * sum)
}

