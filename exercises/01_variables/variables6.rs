// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.
use std::process::Command; 

const NUMBER:i32 = 3;
static z:i32=2;
fn main() {
    println!("Number {}", NUMBER+z);
    let branch = Command::new("git")
                            .arg("rev-parse")
                            .arg("--abbrev-ref")
                            .arg("HEAD")
                            .output()
                            .unwrap();
    let branch_name = String::from_utf8(branch.stdout).unwrap();
    println!("Branch name: {}", branch_name)

}
