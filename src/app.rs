mod file_handler;
mod github;
use chrono::prelude::Local;
use std::path::Path;

use file_handler::{append_string_to_file, get_url_from_txt_file, is_exist_dir};
use github::{
    cleanup, clone_repository, commit_repository, get_metadata_from_url, push_repository,
    GitMetadata,
};

pub fn run() {
    const PROJECT_INFO_FILE_NAME: &str = "project_info.txt";
    const COMMIT_LOG_FILE_NAME: &str = "auto.txt";

    let url = get_url_from_txt_file(PROJECT_INFO_FILE_NAME)
        .trim()
        .to_owned();

    let metadata = get_metadata_from_url(&url);
    let repo_path = &metadata.repo_name;
    let log_file_path = format!("{repo_path}/{COMMIT_LOG_FILE_NAME}");

    if let Err(e) = clone_repository(&url) {
        panic!("Clone repo failed {e}");
    }

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let commit_message: &str = now.as_str();
    append_string_to_file(&log_file_path, commit_message);

    if let Err(e) = commit_repository(&repo_path, commit_message) {
        panic!("Commit repo failed {e}");
    };

    if let Err(e) = push_repository(repo_path) {
        panic!("Push repo failed {e}");
    }

    cleanup(&repo_path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_success() {
        const REPO_URL: &str = "https://github.com/KGESH/qraps-expert-system-example";
        const COMMIT_LOG_FILE_NAME: &str = "auto-test.txt";
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let commit_message: &str = now.as_str();

        let metadata = get_metadata_from_url(REPO_URL);
        let repo_path = &metadata.repo_name;
        let log_file_path = format!("{}/{}", repo_path, COMMIT_LOG_FILE_NAME);

        if let Err(e) = clone_repository(REPO_URL) {
            panic!("Clone repo failed {e}");
        }
        append_string_to_file(&log_file_path, commit_message);

        if let Err(e) = commit_repository(repo_path, commit_message) {
            panic!("Commit repo failed {e}");
        }

        if let Err(e) = push_repository(repo_path) {
            panic!("Push repo failed {e}");
        }
        assert_eq!(is_exist_dir(&repo_path), true);

        cleanup(&repo_path);
        assert_eq!(is_exist_dir(&repo_path), false);
    }
}
