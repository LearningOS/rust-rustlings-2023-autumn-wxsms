fn fib_loop(n: u8) -> u32 {
    let mut a = 1u32;
    let mut b = 1u32;
    let mut i = 1u8;

    loop {
        if i >= n {
            return a;
        }
        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }
}

fn fib_while(n: u8) -> u32 {
    let mut a = 1u32;
    let mut b = 1u32;
    let mut i = 1u8;

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    return a;
}

fn fib_for(n: u8) -> u32 {
    let mut a = 1u32;
    let mut b = 1u32;

    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }

    return a;
}

fn main() {
    println!(
        "{} {} {} {}",
        fib_loop(1),
        fib_loop(2),
        fib_loop(3),
        fib_loop(20)
    );
    println!(
        "{} {} {} {}",
        fib_while(1),
        fib_while(2),
        fib_while(3),
        fib_while(20)
    );
    println!(
        "{} {} {} {}",
        fib_for(1),
        fib_for(2),
        fib_for(3),
        fib_for(20)
    );
}
