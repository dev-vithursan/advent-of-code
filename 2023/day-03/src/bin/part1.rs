use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file_path: &str = "src/bin/input.txt";

    let file: File = File::open(file_path).unwrap();

    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();
    
    let mut data: Vec<Vec<char>> = vec![];
    
    let mut sum: u32 = 0;

    loop {
        let ln: usize = reader.read_line(&mut line).unwrap();

        if ln == 0 {
            break;
        }

        let line_data: Vec<char> = line.trim().chars().collect();

        data.push(line_data);
        line.clear();
    }

    for y in 0..data.len() {

        let mut first_index: usize = 0;
        let mut last_index: usize = 0;
        let mut number_to_add: usize = 0;
        let mut num_found : bool = false;

        for x in 0..data[y].len() {
            println!("{}", &data[y][x]);
            if is_numeric(&data[y][x]) && !num_found {
                first_index = x;
                last_index = x;
                num_found = true;
                number_to_add = data[y][x].to_string().parse::<usize>().unwrap();
            }
            else if is_numeric(&data[y][x]) && num_found {
                number_to_add = (number_to_add * 10) + data[y][x].to_string().parse::<usize>().unwrap();
                last_index = x;

                if x == &data[y].len() - 1 && number_to_add != 0 {
                    //println!("number to process - {} with first index : {} & last index : {}.... {}", number_to_add, first_index, last_index, y);
                    let number_hit: bool = process_number(&data, y, first_index, last_index);
                    if number_hit {
                        println!("number hit! {}", number_to_add);
                        sum += number_to_add as u32;

                        println!("");
                    }
                }
            }
            else {
                if number_to_add != 0 {
                    //println!("number to process - {} with first index : {} & last index : {}.... {}", number_to_add, first_index, last_index, y);
                    let number_hit: bool = process_number(&data, y, first_index, last_index);
                    if number_hit {
                        println!("number hit! {}", number_to_add);
                        sum += number_to_add as u32;

                        println!("");
                    }
                }
                number_to_add = 0 as usize;
                first_index = 0 as usize;
                num_found = false;
            }
        }
    }
    
    println!("number - {}", sum);

}

fn process_number(data: &Vec<Vec<char>>, y_index: usize, number_start: usize, number_end: usize) -> bool {
    println!("processing number: y: {}, start: {}, end: {}", y_index, number_start, number_end);
    let mut found: bool;

    let mut left: char = '.';
    if number_start > 0 as usize {
        left = data[y_index][number_start - 1];
    }

    let mut right: char = '.';
    if number_end < data[0].len() - 1 {
        right = data[y_index][number_end + 1];
    }

    if is_char_symbol(&left) {
        found = true;
    }
    else if is_char_symbol(&right){
        found = true;
    }
    else {
        let start_index: usize;
        let end_index: usize;

        if number_start > 0 as usize {
            start_index = number_start - 1;
        }else {
            start_index = number_start;
        }

        if number_end < data[0].len() - 1 {
            end_index = number_end + 2;
        }else {
            end_index = number_end + 1;
        }
        
        if y_index > 0 {
            let data_slice_top = &data[y_index - 1][start_index..end_index].to_vec();
            println!("u {:?}", data_slice_top);
            found = find_symbol(&data_slice_top);
        }
        else {
            found = false;
        }

        let num_slice = &data[y_index][start_index..end_index].to_vec();
        println!("n {:?}", num_slice);

        if !found {
            if y_index < data[0].len() - 1 {
                let data_slice_bottom = &data[y_index + 1][start_index..end_index].to_vec();
                println!("l {:?}", data_slice_bottom);
                found = find_symbol(&data_slice_bottom);
            }
            else {
                found = false;
            }
        }
    }

    if found {
        return true;
    }

    return false;

}

fn find_symbol(data_slice: &Vec<char>) -> bool {
    // println!("dataa slice {:?}", data_slice);
    let period: char = '.';
    for c in data_slice.iter() {
        if c != &period && !is_numeric(c) {
            return true;
        }
    }
    return false;
}

fn is_char_symbol(c: &char) -> bool {
    let period: char = '.';
    if c != &period && !is_numeric(c) {
        return true;
    }
    return false;
}

fn is_numeric(c: &char) -> bool {
    c.is_numeric()
}
