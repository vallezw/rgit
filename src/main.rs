use std::{fs, process::Command};

fn main() {
    println!("Hello, world!");
    //let mut _res = fs::create_dir("./git");
    //_res = fs::create_dir("./git/test2.git");

    //println!("{:?}", _res);


    let status = Command::new("git")
        .args(["init", "--bare"])
        .current_dir("./git/test.git")
        .status()
        .expect("ls command failed to start");

    println!("ls status: {status}")

}
