use std::os::unix::process::CommandExt;
use std::process::Command;
fn main() {
    test_echo();
    git_add();
    git_commit();
    git_push();
}
fn git_add() {
    println!("Git Add files.");
}
fn git_commit() {
    println!("Git Commit. ");
}
fn git_push() {
    println!("Git Push");
}

fn test_echo() {
    let err = Command::new("echo").arg("hello").arg("world").exec();
    println!("Error: {}", err)
}
