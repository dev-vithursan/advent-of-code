use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_path: &str = "src/bin/input.txt";
    let file: File = File::open(file_path).unwrap();

    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();
    let mut arr = Vec::new();
    let mut sum: i32 = 0;

    loop {
        let ln: usize = reader.read_line(&mut line).unwrap();

        if ln == 0 {
            break;
        }

        // loop through the chars, integer found the store the first and last.
        for char in line.chars() {
            if char.is_numeric() {
                arr.push(char);
            };
        }

        // get the first and last index from the vector and concatnate them, convert into integer
        let res = format!("{}{}", arr[0], arr[arr.len() - 1]);
        sum += res.parse::<i32>().unwrap();

        arr.clear();
        line.truncate(0);
    }

    println!("Sum : {}", sum);
}
