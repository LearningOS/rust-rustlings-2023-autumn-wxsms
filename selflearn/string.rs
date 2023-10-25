fn main() {
    let mut s = String::from("123");
    s.push_str("456");
    s += "789";

    dbg!(&s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = format!("{s1}-{s2}-{s3}");
    dbg!(&s4);

    dbg!(String::from("1234").len());
    dbg!(String::from("你").len());
    dbg!(String::from("你2").len());
    dbg!(String::from("你好").len());
    dbg!(String::from("Здравствуйте").len());

    dbg!(&String::from("你好")[0..3]);

    for c in "你好".chars() {
        println!("{c}");
    }
}
