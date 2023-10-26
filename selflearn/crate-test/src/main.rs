mod animals;
mod fishes;

fn main() {
    println!("Hello, world!");
    animals::life::walk();
    animals::life::eat();

    fishes::life::walk();
    fishes::life::eat();
}
