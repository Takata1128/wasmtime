use clap::Parser;

#[derive(Parser)]
struct Args {
    fib_n: usize,
    // leibniz_n: usize,
}

fn main() {
    let args = Args::parse();
    // fib(args.fib_n);
    fib_rec(args.fib_n);
    // leibniz(args.leibniz_n);
}

fn fib(n: usize) {
    let mut v: Vec<i32> = vec![0; n + 1];
    v[0] = 1;
    v[1] = 1;
    for i in 2..n + 1 {
        v[i] = (v[i - 1] + v[i - 2]) % 10_000;
    }
    println!("{}", v[n]);
}

fn fib_rec(n: usize) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let ret = fib_rec(n - 1) + fib_rec(n - 2);
        ret
    }
}

// 竹内関数
fn tarai(x: i32, y: i32, z: i32) -> i32 {
    if x <= y {
        y
    } else {
        tarai(tarai(x - 1, y, z), tarai(y - 1, z, x), tarai(z - 1, x, y))
    }
}

fn leibniz(n: usize) {
    let mut sum: f64 = 0.0;
    let mut signum = 1;
    let mut denom = 1.0;
    for _ in 0..n {
        sum += signum as f64 / denom;
        signum *= -1;
        denom += 2.0;
    }
    println!("{}", sum);
}
