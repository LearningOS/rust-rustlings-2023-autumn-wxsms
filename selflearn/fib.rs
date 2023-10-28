fn fib_next(a: u32, b: u32) -> (u32, u32) {
    (b, a + b)
}

fn fib_loop(n: u8) -> u32 {
    let mut a = 1u32;
    let mut b = 1u32;
    let mut i = 1u8;
    loop {
        if i >= n {
            return a;
        }
        (a, b) = fib_next(a, b);
        i += 1;
    }
}

fn fib_while(n: u8) -> u32 {
    let mut a = 1u32;
    let mut b = 1u32;
    let mut i = 1u8;
    while i < n {
        (a, b) = fib_next(a, b);
        i += 1;
    }
    return a;
}

fn fib_for(n: u8) -> u32 {
    let mut a = 1u32;
    let mut b = 1u32;
    for _ in 1..n {
        (a, b) = fib_next(a, b);
    }
    return a;
}

fn fib_rec(n: u8) -> u32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fib_rec(n - 1) + fib_rec(n - 2),
    }
}

fn main() {
    println!(
        "loop: {} {} {} {}",
        fib_loop(1),
        fib_loop(2),
        fib_loop(3),
        fib_loop(20)
    );
    println!(
        "while: {} {} {} {}",
        fib_while(1),
        fib_while(2),
        fib_while(3),
        fib_while(20)
    );
    println!(
        "for: {} {} {} {}",
        fib_for(1),
        fib_for(2),
        fib_for(3),
        fib_for(20)
    );
    println!(
        "rec: {} {} {} {}",
        fib_rec(1),
        fib_rec(2),
        fib_rec(3),
        fib_rec(20)
    );
}
