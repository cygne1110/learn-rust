fn main() {
    testfibo();
}

fn testfibo() {
    let n: isize = 10;
    let f: isize = fibo(n);

    println!("Fibonacci number {n} is {f}");
}

fn fibo(n: isize) -> isize {
    if n < 0 {
        panic!("No!!!");
    } else {
        match n {
            0 => 0,
            1 => 1,
            _ => fibo(n - 1) + fibo(n - 2),
        }
    }
}
