use std::io;

struct Equat {
    left: u32,
    operater: String,
    right: u32
}

fn main() {
    let mut num = String::new();
    println!("equation: ");
    let _equation = io::stdin().read_line(&mut num).unwrap();
    let split: Vec<&str> = num.split_whitespace().collect();

    let left: u32;
    let operater: String;
    let right: u32;

    let temp: u32 = split[0].parse().unwrap();
    left = temp;
    operater = split[1].to_string();
    let temp: u32 = split[2].parse().unwrap();
    right = temp;
    let struc = Equat{left, operater, right};
    let mut result: u32 = 0;

    match split[1] {
        "+" => result = add(struc.right, struc.left),
        "-" => result = subtract(struc.right, struc.left),
        "*" => result = multiply(struc.right, struc.left),
        "/" => result = divide(struc.right, struc.left),
        _ => {}
    };

    println!("{}", result);
}

fn add(right: u32, left: u32) -> u32{
    let result: u32 = right + left;
    result
}

fn subtract(right: u32, left: u32) -> u32{
    let result: u32 = right - left;
    result
}

fn multiply(right: u32, left: u32) -> u32{
    let result: u32 = right * left;
    result
}

fn divide(right: u32, left: u32) -> u32{
    let result: u32 = right / left;
    result
}
