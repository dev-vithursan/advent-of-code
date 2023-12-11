use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

fn main() {
    let file_path = "src/bin/input.txt";

    let file: File = File::open(file_path).unwrap();

    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();

    let mut vec_matches: Vec<u32> = vec![0; 210]; //Should read this from the file - number of games (210)

    let mut index: usize = 0;

    loop {
        let ln: usize = reader.read_line(&mut line).unwrap();
        if ln == 0 {
            break;
        }

        let turn_nums = line.trim().split(":").nth(1).unwrap();

        let winning_nums: HashSet<u32> = turn_nums.split(" | ").nth(0).unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect(); 
        let our_nums: HashSet<u32> = turn_nums.split(" | ").nth(1).unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        let intersect: HashSet<_> = winning_nums.intersection(&our_nums).collect();

        vec_matches[index] += 1;

        if !intersect.is_empty() {
            for i in 0..intersect.len() {
                vec_matches[index + i + 1] += vec_matches[index];
            }
        }

        index += 1;
        line.clear();
    }
    
    let sum: u32 = vec_matches.iter().sum();
    println!("Sum : {}", sum);
}
