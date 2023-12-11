use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

fn main() {
    let file_path = "src/bin/input.txt";

    let file: File = File::open(file_path).unwrap();

    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();

    let mut sum: u32 = 0;
    let base: u32 = 2;

    loop {
        let ln: usize = reader.read_line(&mut line).unwrap();
        if ln == 0 {
            break;
        }

        let game = line.trim().split(":").nth(0).unwrap();
        let turn_nums = line.trim().split(":").nth(1).unwrap();

        let winning_nums: HashSet<u32> = turn_nums.split(" | ").nth(0).unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect(); 
        let our_nums: HashSet<u32> = turn_nums.split(" | ").nth(1).unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        let intersect: HashSet<_> = winning_nums.intersection(&our_nums).collect();

        if !intersect.is_empty() {
            sum += base.pow((intersect.len() as u32) -1);
        }

        line.clear();
    }

    println!("Sum : {}", sum);
}
