use std::os::unix::process::CommandExt;
use std::process::Command;
fn main() {
    //  git add .
    println!("git add");
    git_add();
    println!("git commit");
    git_commit();
    println!("git push");
    git_push();
}
fn git_add() {
    let err = Command::new("git").arg("add").arg(".").exec();
    println!("Error: method git add . : {}", err);
}
fn git_commit() {
    //  git commit
    //  git commit -m "{default_value}"
}
fn git_push() {
    //  git push -u
    let err = Command::new("git").arg("push").arg("-u").exec();
    println!("Error: method git push -u : {}", err);
}

//fn test_echo() {
//    let err = Command::new("echo").arg("hello").arg("world").exec();
//    println!("Error: {}", err)
//}
