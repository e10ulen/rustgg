extern crate log;
use log::warn;
use std::os::unix::process::CommandExt;
use std::process::Command;
fn main() {
    println!("Git Add files.");
    let err = Command::new("git").arg("add").arg(".").exec();
    warn!("Error: method git add : {}", err);

    println!("Git Commit. ");
    println!("Git Push");
    let err = Command::new("git").arg("push").arg("-u").exec();
    println!("Error: method git push -u : {}", err)
}

//fn test_echo() {
//    let err = Command::new("echo").arg("hello").arg("world").exec();
//    println!("Error: {}", err)
//}
