use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

fn main() {
    let file_path: &str = "src/bin/input.txt";

    let file: File = File::open(file_path).unwrap();

    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();
    
    let mut data: Vec<Vec<char>> = vec![];

    let mut gear_set: HashMap<String, Vec<u32>> = HashMap::new();
    
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

        let mut last_index: usize = 0;
        let mut first_index: usize = 0;
        let mut number_to_add: usize = 0;
        let mut num_found : bool = false;

        for x in 0..data[y].len() {
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
                    let (star_found, star_pos) = process_number(&data, y, first_index, last_index);
                    if star_found {
                        gear_set.entry(star_pos.to_string()).or_default().push(number_to_add as u32);
                    }
                }
            }
            else {
                if number_to_add != 0 {
                    let (star_found, star_pos) = process_number(&data, y, first_index, last_index);
                    if star_found {
                        gear_set.entry(star_pos.to_string()).or_default().push(number_to_add as u32);
                    }
                }

                number_to_add = 0 as usize;
                first_index = 0 as usize;
                last_index = 0 as usize;
                num_found = false;
            }
        }
    }

    for (_k , v) in gear_set.iter() {
        if v.len() == 2 {
            sum += v[0] * v[1];
        }
    }

    println!("{}", sum);

}

fn process_number(data: &Vec<Vec<char>>, y_index: usize, number_start: usize, number_end: usize) -> (bool, f32) {
    let mut left: char = '.';
    if number_start > 0 as usize {
        left = data[y_index][number_start - 1];
    }

    let mut right: char = '.';
    if number_end < data[0].len() - 1 {
        right = data[y_index][number_end + 1];
    }

    if is_char_star(&left) {
        let star_pos: f32 = get_pos_as_flot(y_index, number_start - 1, 0);
        println!("star_pos {}", star_pos);
        return (true, star_pos);
    }
    else if is_char_star(&right){
        let star_pos: f32 = get_pos_as_flot(y_index, number_end, 1);
        println!("star_pos {}", star_pos);
        return (true, star_pos);
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
            let star_index: Option<usize> = find_star(&data_slice_top);
            match star_index {
                Some(x) => {
                    let star_pos_top: f32 = get_pos_as_flot(y_index - 1, start_index, x);
                    println!("star_pos {}", star_pos_top);
                    return (true, star_pos_top);
                },
                None => {
                    println!("Star not found!");
                },
            }
        }

        if y_index < data[0].len() - 1 {
            let data_slice_bottom = &data[y_index + 1][start_index..end_index].to_vec();
            let star_index: Option<usize> = find_star(&data_slice_bottom);
            match star_index {
                Some(x) => {
                    let star_pos_bottom: f32 = get_pos_as_flot(y_index + 1, start_index, x);
                    println!("star_pos {}", star_pos_bottom);
                    return (true, star_pos_bottom);
                },
                None => {
                    println!("Star not found!");
                },
            }
        }
    }

    return (false, 0 as f32);

}


fn find_star(data_slice: &Vec<char>) -> Option<usize> {
    let star: char = '*';
    for (index, c) in data_slice.iter().enumerate() {
        if c == &star {
            return Some(index);
        }
    }
    return None;
}

fn is_char_star(c: &char) -> bool {
    let star: char = '*';
    if c == &star {
        return true;
    }
    return false;
}

fn is_numeric(c: &char) -> bool {
    c.is_numeric()
}

fn get_pos_as_flot(y: usize, x: usize, pos: usize) -> f32 {
    let float_str: String = format!("{}.{}", y, x + pos);
    let float_val = float_str.parse::<f32>().unwrap();
    return float_val;
}
