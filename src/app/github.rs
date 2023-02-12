use std::process;
use std::process::ExitStatus;

pub struct GitMetadata {
    pub username: String,
    pub repo_name: String,
}

pub fn get_metadata_from_url(url: &str) -> GitMetadata {
    let tokens: Vec<&str> = url.split("/").collect();
    let repo_name = tokens[tokens.len() - 1];
    let username = tokens[tokens.len() - 2];

    GitMetadata {
        username: String::from(username),
        repo_name: String::from(repo_name),
    }
}

pub fn clone_repository(url: &str) -> std::io::Result<ExitStatus> {
    let clone_command = "git clone ".to_owned() + url;
    let mut clone_child = process::Command::new("sh")
        .arg("-c")
        .arg(clone_command)
        .spawn()
        .unwrap();

    clone_child.wait()
}

pub fn commit_repository(dir_name: &str, commit_message: &str) -> std::io::Result<ExitStatus> {
    process::Command::new("sh")
        .arg("-c")
        .current_dir(dir_name)
        .arg("git add .")
        .spawn()
        .unwrap()
        .wait();

    process::Command::new("sh")
        .arg("-c")
        .current_dir(dir_name)
        .arg(format!("git commit -m '{commit_message}'"))
        .spawn()
        .unwrap()
        .wait()
}

pub fn push_repository(dir_name: &str) -> std::io::Result<ExitStatus> {
    process::Command::new("sh")
        .arg("-c")
        .current_dir(dir_name)
        .arg("git push origin main")
        .spawn()
        .unwrap()
        .wait()
}

pub fn cleanup(dir_name: &str) {
    let remove_result = process::Command::new("sh")
        .arg("-c")
        .arg(format!("rm -rf {dir_name}"))
        .spawn()
        .unwrap()
        .wait();
}
