use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_num: f64 = first.parse().unwrap();
    let second_num: f64 = second.parse().unwrap();

    let result = operate(operator, first_num, second_num);

    println!("{} {} {} = {}", first, operator, second, result);
}

fn operate(operator: char, first_n: f64, second_n: f64) -> f64 {
    match operator {
        '+' => first_n + second_n,
        '*' | 'x' | 'X' => first_n * second_n,
        '-' => first_n - second_n,
        '/' => first_n / second_n,
        _ => panic!("Invalid opertor used!"),
    }
}
