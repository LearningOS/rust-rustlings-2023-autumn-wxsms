#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn square(size: u32) -> Self {
        Rect {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, r2: &Rect) -> bool {
        self.width >= r2.width && self.height >= r2.height
    }
}

// fn area(r: &Rect) -> u32 {
//     r.width * r.height
// }

fn main() {
    let scale = 2;
    let r = Rect {
        width: dbg!(30 * scale),
        height: dbg!(40),
    };

    dbg!(r.area());

    let r2 = Rect {
        width: 10,
        height: 20,
    };
    println!("r1 can hold r2: {}", r.can_hold(&r2));
    println!("r2 can hold r1: {}", r2.can_hold(&r));

    let r3 = Rect::square(100);
    dbg!(&r3);
}
