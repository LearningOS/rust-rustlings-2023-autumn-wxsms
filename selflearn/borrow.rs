fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
    // println!("{} and {}", r1, r2);

    let r4 = &r3;
    println!("{}", r4);

    *r3 = String::from("123");
    println!("{}", r3);
}
