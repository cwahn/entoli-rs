use entoli::prelude::Io;
use entoli::prelude::{foldl, get_line, iterate, last, put_str_ln, scanl1, take};

pub fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    let main = put_str_ln("Input natural number:")
        .then(get_line)
        .and_then(|s| {
            let n = s.parse().unwrap();
            put_str_ln(format!("fib({}) = {}", n, fib(n)))
        });

    main.run();
}
