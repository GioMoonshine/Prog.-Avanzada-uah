fn factorial(n:i32) -> i32 {
    let mut fact = 1;
    for i in 1..n+1 {
        fact *= i;
    }
    return fact;
}

fn factorial_rec(n:i32) -> i32 {
    if n == 0 {
        return 1;
    }
    let fact = n * factorial_rec(n-1);
    return fact
}

fn fibonacci(n:i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let fib = fibonacci(n-1) + fibonacci(n-2);
    return fib;
}

fn main() {
    println!("{}",factorial(6));
    println!("{}",factorial(4));
    println!("{}",factorial(0));
    println!("{}",factorial_rec(6));
    println!("{}",factorial_rec(4));
    println!("{}",factorial_rec(0));
    println!("{}",fibonacci(6));
    println!("{}",fibonacci(4));
    println!("{}",fibonacci(0));
}
