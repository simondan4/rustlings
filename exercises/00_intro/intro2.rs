// intro2.rs
//
// Make the code print a greeting to the world.
//
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut line = String::new();
    println!("Hello there! Enter your Name:");
    let test = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello {}!", line);
    println!("This is my first print statement");
    println!("This is my second print statement")
}
