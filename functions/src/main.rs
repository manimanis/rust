use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    process_numbers(&[1, 2, 3, 4, 5]);

    let chunk = split_str("hello, world!".to_string(), ',', 1);
    println!("Split string: {}", chunk);

    let nums = [100, 200, 300, 400, -20];
    let res = sum(&nums);
    println!("The sum is : {}", res);

    // loop_and_panic(&nums);

    file_reader();
}

fn process_numbers(numbers: &[i32]) {
    let mut s = 0;
    for number in numbers {
        s += number;
    }

    println!("The sum of the numbers is: {}", s);

    if s % 2 == 0 {
        println!("{} is even!", s);
    } else {
        println!("{} is odd!", s);
    }
}

fn split_str(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);
    result.expect("Oops!").to_string()
}

fn sum(numbers: &[i32]) ->i32{
    let mut s = 0;
    for num in numbers {
        s += num;
    }
    s
}

fn loop_and_panic(numbers: &[i32]) {
    for num in numbers {
        if *num < 0 {
            panic!("Negative number {}", num);
        }
        println!("Number: {}", num);
    }
}

fn file_reader() {
    let file = File::open("filename.txt");
    let file = match file {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", err);
                }
                _ => {
                    panic!("Error opening file: {}", err);
                }
            }
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}