fn upper(s: &String) -> String {
    s.to_uppercase()
}

fn first_world(s: &str) -> &str {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    let a = String::from("abc");
    let b = upper(&a);
    println!("a:{a}, b:{b}");

    let c = b + " hello MOTO";
    println!("c:{c}");

    let world = first_world(&c);
    println!("world {world}");

    let world_str = world.to_string();
    println!("world_str {world_str}");


    // c.clear();
}
