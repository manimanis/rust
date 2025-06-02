use std::{fmt::Debug, io};

fn main() {
    // variable_assign();
    // control_flow();
    // shadowing();
    // loops();
    // conditionals();
    // arrays();
    slices();
}

fn variable_assign() {
    let message = "Name: Mohamed Anis, Weight: ";
    let weight = 190.0;
    let kilos = weight / 2.2;

    println!("{}{}", message, weight);
    println!("{}{}", message, kilos);

    let mut msg2 = String::from("Name: Mohamed Anis, Weight: ");
    msg2.clear();
    let mut height = 180;
    height = 175;
    println!("{}{}", msg2, height);
}

fn control_flow() {
    let proceed = true;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 190;
    if height > 180 {
        println!("Average");
    } else {
        println!("Short");
    }
}

fn shadowing() {
    let height = 200;
    let height = height - 20;
    let result = if height > 180 {
        "Tall"
    } else if height > 170 {
        "Average"
    } else {
        "Short"
    };
    println!("Result: {}", result);

    let health = if height < 180 { "Good" } else { "Unknown" };
    println!("Health: {}", health);

    let health = if height < 180 { true } else { false };
    println!("Health: {}", health);
}

fn loops() {
    let mut x = 1;
    loop {
        println!("x is {}", x);
        x += 1;
        if x > 5 {
            break;
        }
    }

    while x <= 10 {
        println!("x is {}", x);
        x += 1;
    }

    for i in 10..20 {
        println!("i = {}", i);
    }

    for i in (20..=30).rev() {
        println!("i = {}", i);
    }

    let numbers = vec![40, 50, 60, 70];
    for n in numbers {
        println!("n = {}", n);
    }

    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word (type 'stop' to exit):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        println!("You entered: {}", input);
    }
    println!("Goodbye!");
}

fn conditionals() {
    let maybe_num: Option<Option<()>> = Some(None);
    // let maybe_num = Some(30);
    if let Some(number) = maybe_num {
        println!("The number is {:?}", number);
    } else {
        println!("There is no number");
    }

    let a = 5;
    let b = 6;
    let msg = if a > b {
        "a > b"
    } else if a < b {
        "a < b"
    } else {
       "a < b" 
    }; 
    println!("{}", msg);
}

fn arrays() {
    let array = [10, 20, 30, 40, 50];
    println!("Array length is {}", array.len());

    for i in 0..array.len() {
        println!("t[{}] = {}", i, array[i]);
    }

    let mut s = 0;
    let mut max = 0;
    let mut min = 0;
    for i in 0..array.len() {
        s += array[i];
        max = if array[i] > array[max] { i } else { max };
        min = if array[i] < array[min] { i } else { min };
    }
    println!("sum = {}", s);
    println!("max = {}", array[max]);
    println!("min = {}", array[min]);
}

fn slices() {
    let nbs = [10, 20, 30, 40, 50, 60];
    let slc = &nbs[2..4];
    
}