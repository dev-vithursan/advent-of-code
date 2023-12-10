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
    let mut sum: u32 = 0;

    loop {
        let ln: usize = reader.read_line(&mut line).unwrap();

        if ln == 0 {
            break;
        }

        // loop through the chars, integer found the store the first and last.
        let mut index = 0;
        for char in line.chars() {
            if char.is_numeric() {
                arr.push(char);
            } else {
                let reduced_line = &line[index..];

                if reduced_line.starts_with("one") {
                    arr.push('1');
                } else if reduced_line.starts_with("two") {
                    arr.push('2');
                } else if reduced_line.starts_with("three") {
                    arr.push('3');
                } else if reduced_line.starts_with("four") {
                    arr.push('4');
                } else if reduced_line.starts_with("five") {
                    arr.push('5');
                } else if reduced_line.starts_with("six") {
                    arr.push('6');
                } else if reduced_line.starts_with("seven") {
                    arr.push('7');
                } else if reduced_line.starts_with("eight") {
                    arr.push('8');
                } else if reduced_line.starts_with("nine") {
                    arr.push('9');
                }
            };
            index += 1;
        }

        // get the first and last index from the vector and concatnate them, convert into integer
        let res = format!("{}{}", arr[0], arr[arr.len() - 1]);
        sum += res.parse::<u32>().unwrap();

        arr.clear();
        line.truncate(0);
    }

    println!("Sum : {}", sum);
}
