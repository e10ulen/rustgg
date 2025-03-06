use std::os::unix::process::CommandExt;
use std::process::Command;
extern crate clap;
//fn main() {
//  git add .
//    println!("git add");
//    git_add();
//    println!("git commit");
//    git_commit();
//    println!("git push");
//    git_push();
//}
//fn git_add() {
//    let err = Command::new("git").arg("add").arg(".").exec();
//    println!("Error: method git add . : {}", err);
//}
//fn git_commit() {
//  git commit
//  git commit -m "{default_value}"
//}
//fn git_push() {
//  git push -u
//    let err = Command::new("git").arg("push").arg("-u").exec();
//    println!("Error: method git push -u : {}", err);
//}
use clap::{Parser, Subcommand};
use std::string::String;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Clis {
    //  git add .するための前準備
    #[clap(help = "ファイルをaddする", short = 'a', long = "add")]
    add_file: Option<String>,

    //  コミットメッセージ受付の下準備
    #[clap(help = "コミットメッセージを入力させたい", short, long = "commit")]
    commit: Option<String>,

    //  プッシュするための下準備
    #[clap(help = "ファイルをプッシュする", short = 'p', long = "push")]
    file_push: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

// fn main() {
//     let cli = Clis::parse();
//     prsintln!("args:{}", cli.add_file);
//     println!("args:{}", cli.commit);
//     println!("args:{}", cli.file_push);
// }
// //fn test_echo() {
// //
// //    let err = Command::new("echo").arg("hello").arg("world").exec();
// //    println!("Error: {}", err)
// //}
//use std::path::PathBuf;

// struct Cli {
//     /// Optional name to operate on
//     name: Option<String>,

//     /// Sets a custom config file
//     #[arg(short, long, value_name = "FILE")]
//     config: Option<PathBuf>,

//     /// Turn debugging information on
//     #[arg(short, long, action = clap::ArgAction::Count)]
//     debug: u8,

//     #[command(subcommand)]
//     command: Option<Commands>,
// }

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Clis::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.add_file.as_deref() {
        println!("Value for name: {name}");
        let err = Command::new("git").arg("add").arg(".").exec();
        println!("Error: method git add . : {}", err);
    }

    if let Some(config_path) = cli.commit.as_deref() {
        println!("Value for config: {}", config_path);
        let err = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(config_path)
            .exec();
        println!("Error: method git commit -m {}", err);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    if let Some(files) = cli.file_push.as_deref() {
        println!("Value for push: {}", files);
        let err = Command::new("git").arg("push").arg("-u").exec();
        println!("Error: method git push -u : {}", err);
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}
