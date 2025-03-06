use chrono::Utc;
use std::process::Command;
use std::string::String;
fn main() {
    let day_time = Utc::now()
        .format("%Y年%m月%d日 %H時%M分%S秒 %Z")
        .to_string();
    let msg = "[fix]".to_owned() + &day_time;
    //  全てのコマンドを自動実行する
    let com1 = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("filed:add");
    let com2 = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(msg)
        .output()
        .expect("feild:commit");
    let com3 = Command::new("git")
        .arg("push")
        .output()
        .expect("feild:push");
    println!("git add {}", String::from_utf8_lossy(&com1.stdout));
    println!("git commit {}", String::from_utf8_lossy(&com2.stdout));
    println!("git push {}", String::from_utf8_lossy(&com3.stdout));
}
