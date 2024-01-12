// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let value = call_me(4);
    println!("Passed number {}", value)
}

fn call_me(num: u32) -> u32 {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
    return num
}
