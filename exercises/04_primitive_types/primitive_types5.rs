// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;
    use std::time::Instant;
    let now = Instant::now();
    let mut x = 0;
    let num = 100_000_000;
    for i in 0..num {
        x = x+1;
    }
    println!("x is {}", x);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
