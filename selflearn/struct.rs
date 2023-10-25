struct User {
    email: String,
    password: String,
    age: i32,
}

fn main() {
    let u1 = User {
        email: String::from("email"),
        password: String::from("password"),
        age: 123,
    };

    println!("{}, {}, {}", u1.email, u1.password, u1.age);

    let u2 = User {
        email: String::from("123"),
        age: 321,
        ..u1
    };
    println!("{}, {}, {}", u2.email, u2.password, u2.age);
    println!("{}, {}", u1.email, u1.age);
}
