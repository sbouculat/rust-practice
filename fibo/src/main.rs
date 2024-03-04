use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let value = args[1].parse::<i32>().unwrap();

    println!("F{} = {}", value, fibo(value));
}

fn fibo(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibo(n-2) + fibo(n-1),
    }
}
