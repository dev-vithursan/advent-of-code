use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_path: &str = "src/bin/input.txt";
    let file: File = File::open(file_path).unwrap();

    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();

    let mut possible_games = Vec::new();

    let mut sum: u32 = 0;

    loop {
        let ln: usize = reader.read_line(&mut line).unwrap();

        if ln == 0 {
            break;
        }

        let line_str: &str = &mut line.as_str();
        let [game, draws]: [&str; 2] = collon_spliter(line_str);

        possible_games.push(game.to_string());
        
        let turns: Vec<&str> = draws.split("; ").collect();

        let mut exceeded = false;

        for turn in turns.iter() {
            let pairs: Vec<&str> = turn.trim().split(", ").collect();
            for pair in pairs {
                // println!("pair : {}", pair);
                let [count, colour] = whitespace_spliter(pair);
                exceeded = match colour {
                    "red" => count.parse::<u32>().unwrap() > 12,
                    "green" => count.parse::<u32>().unwrap() > 13,
                    "blue" => count.parse::<u32>().unwrap() > 14,
                    _ => false,
                };

                if exceeded{
                    println!("{}, exceeded - {}", game, exceeded);
                    break;
                }
            }

            if exceeded {
                break;
            }
        }

        if exceeded {
            possible_games.pop();
        }

        line.clear();
    }

    for pg in possible_games.iter() {
        let [_, id] = whitespace_spliter(pg);
        sum += id.parse::<u32>().unwrap();
    }

    println!("Sum : {}", sum);
}

fn collon_spliter(line: &str) -> [&str; 2] {
    line.trim()
        .split(":")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default()
}

fn whitespace_spliter(line: &str) -> [&str; 2] {
    line.trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default()
}