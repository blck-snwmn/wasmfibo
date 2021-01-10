use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let v = args.get(1).map_or(0, |x| x.parse().unwrap_or_default());
    println!("{:}", fib_main(v));
}

fn fib_main(c: u8) -> u128 {
    fib(c, 1, 0)
}

fn fib(c: u8, n_1: u128, n_2: u128) -> u128 {
    match c {
        0 => n_2,
        1 => n_1,
        _ => fib(c - 1, n_1 + n_2, n_1),
    }
}

#[test]
fn test_fib() {
    assert_eq!(fib_main(0), 0);
    assert_eq!(fib_main(1), 1);
    assert_eq!(fib_main(2), 1);
    assert_eq!(fib_main(3), 2);
    assert_eq!(fib_main(4), 3);
    assert_eq!(fib_main(5), 5);
    assert_eq!(fib_main(50), 12586269025);
    assert_eq!(fib_main(100), 354224848179261915075);
}
