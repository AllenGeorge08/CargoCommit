use names::Generator;
use std::process::{Command, exit};


//e Add git remote add origin, change branch everything..
fn update_commit_push() {

    //git init
    //git remote -v
    //git remote add origin if doesn't exist
    //git 

    //git add .
    let add_command = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to execute git add command");

    if !add_command.status.success() {
        println!("Error: Failed to add files to the git repo");
        exit(1);
    }

    //git commit -m "Message"
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("Failed to commit");

    if !commit_command.status.success() {
        println!("Error: Failed to commit to the git repo");
        exit(2);
    }

    //git pussh origin master....
    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Failed to push");

    if !push_command.status.success() {
        println!("Error: Failed to push to the git remote repo");
        exit(3);
    }

    println!("Succesfully addded,commited and pushed all changes");
}

fn name_generator() -> String{
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn main() {
    update_commit_push();
    println!("Welcome to the git automate CLI");
}
