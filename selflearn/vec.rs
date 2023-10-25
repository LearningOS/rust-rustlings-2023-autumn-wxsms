fn main() {
    let mut v = vec![1, 2, 3];

    dbg!(&v);

    v.push(4);
    dbg!(&v);
    dbg!(&v[0]);

    dbg!(&v.get(100).is_none());

    for i in &mut v {
        *i *= 2;
    }
    dbg!(&v);
}
