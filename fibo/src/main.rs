fn main() {
    println!("{}", fibo(16));
}

fn fibo(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => (fibo(n-2) + fibo(n-1)),
    }
}
