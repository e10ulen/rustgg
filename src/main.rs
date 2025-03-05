use std::os::unix::process::CommandExt;
use std::process::Command;
fn main() {
    //test_echo();
    git_add();
    git_commit();
    git_push();
}

fn git_add() {
    println!("Git Add files.");
    let err = Command::new("git").arg("add").arg(".").exec();
    println!("Error: method git add : {}", err);
    git_commit();
}

fn git_commit() {
    println!("Git Commit. ");
    git_push();
}
fn git_push() {
    println!("Git Push");
    //let err = Command::new("git").arg("push").arg("-u").exec();
    //println!("Error: method git push -u : {}", err)
}

//fn test_echo() {
//    let err = Command::new("echo").arg("hello").arg("world").exec();
//    println!("Error: {}", err)
//}
