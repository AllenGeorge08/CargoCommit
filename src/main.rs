use names::Generator;
use std::process::{Command, exit};

fn git_init() {
    let init_command = Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to execute git init command");

    if !init_command.status.success() {
        println!("Error: failed to init the repo");
        exit(1);
    }
}

fn git_add_remote_origin(url: &str) {
    let git_add_command = Command::new("git")
        .args(["remote", "add", "origin", url])
        .output()
        .expect("Failed to execute git command");

    if !git_add_command.status.success() {
        println!("Error: failed to add to remote origin");
    }
}

fn git_check_branch_and_change_branch() {
    let git_check_branch = Command::new("git")
        .args(["branch", "-v"])
        .output()
        .expect("Failed to execute git command");

    if !git_check_branch.status.success() {
        println!("Error: failed to check branch");
        exit(3);
    }
}

fn git_add_upstream_url(url: &str) {
    let git_add_upstream_url = Command::new("git")
        .args(["remote", "add", "upstream", url])
        .status()
        .expect("Failed to add upstream url");

    if !git_add_upstream_url.success() {
        println!("Error: Failed to add upstream url");
        exit(4);
    }
}

fn git_rebase() {}

fn git_merge() {}

fn git_pull() {}

fn git_add(){

}

fn git_commit(){

}

fn git_push(){

}



//e Use this in commit
fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}

// fn main() {
//     update_commit_push();
//     println!("Welcome to the git automate CLI");
// }
