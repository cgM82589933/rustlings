// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` for a hint.

fn main() {
    // Solution 1
    let greeting = "world";

    // Solution 2
    // let mut greeting = String::new();
    // greeting = "world".to_string();

    // Solution 3
    // let greeting = String::from("world");

    // Solution 4
    // let greeting: &str = "world";

    println!("Hello {}!", greeting);
}
