use std::{
    fs::File,
    io::{BufRead, BufReader}, vec,
};

fn main() {
    let file_path: &str = "src/bin/input.txt";
    let file: File = File::open(file_path).unwrap();

    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();

    let mut sum: u32 = 0;

    loop {
        let ln: usize = reader.read_line(&mut line).unwrap();

        if ln == 0 {
            break;
        }

        let line_str: &str = &mut line.as_str();
        let [_game, draws]: [&str; 2] = collon_spliter(line_str);
        
        let turns: Vec<&str> = draws.split("; ").collect();
        
        let mut max_colour = vec![0; 3];

        for turn in turns.iter() {
            let pairs: Vec<&str> = turn.trim().split(", ").collect();
            for pair in pairs {
                // println!("pair : {}", pair);
                let [count, colour] = whitespace_spliter(pair);
                match colour {
                    "red" => {
                        let count = count.parse::<u32>().unwrap();
                        if count > max_colour[0] {
                            max_colour[0] = count;
                        }
                    }
                    "green" => {
                        let count = count.parse::<u32>().unwrap();
                        if count > max_colour[1] {
                            max_colour[1] = count;
                        }
                    },
                    "blue" => {
                        let count = count.parse::<u32>().unwrap();
                        if count > max_colour[2] {
                            max_colour[2] = count;
                        }
                    },
                    _ => (),
                };
            }
        }
        
        let mut line_product = 1;
        let zero: u32 = 0;
        for max in max_colour.iter() {
            if max > &zero {
                line_product *= max;
            }
        }
        // println!("line count = {}", line_product);

        sum += line_product;

        line.clear();
    }

    println!("Sum - {}", sum)
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