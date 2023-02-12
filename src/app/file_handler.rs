use std::fs;
use std::io::Read;
use std::io::{Seek, SeekFrom, Write};

pub fn get_url_from_txt_file(file_name: &str) -> String {
    let file = fs::OpenOptions::new().read(true).open(file_name);

    match file {
        Ok(_) => {
            let mut url = String::new();
            file.unwrap().read_to_string(&mut url).unwrap();
            url
        }
        Err(_) => panic!("{}", format!("{} not found!", file_name)),
    }
}

pub fn append_string_to_file(file_name: &str, content: &str) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)
        .unwrap();

    writeln!(file, "{}", &content).expect("Can not append string to file ");
}

pub fn is_exist_dir(dir_name: &str) -> bool {
    match fs::metadata(dir_name) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;

    #[test]
    fn get_repo_url_success() {
        const MOCK_URL: &str = "https://github.com/KGESH/qraps-expert-system-example";
        const FILE_NAME: &str = "project_test_info.txt";
        let repo_url = get_url_from_txt_file(FILE_NAME);

        assert_eq!(repo_url, MOCK_URL);
    }

    #[test]
    fn append_string_to_file_success() {
        // given
        const FILE_NAME: &str = "append-test.txt";

        // when
        let result = append_string_to_file(FILE_NAME, "hello, world!");

        // then
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(FILE_NAME)
            .unwrap();
        let mut written_string = String::new();
        file.read_to_string(&mut written_string).unwrap();

        assert_eq!(written_string, "hello, world!\n");

        // remove content
        remove_file(FILE_NAME);
    }

    fn remove_file(file_name: &str) {
        if let Err(e) = fs::remove_file(file_name) {
            panic!("Remove file error {e}");
        }
    }

    fn clear_content(file_name: &str) {
        let mut file = fs::OpenOptions::new().write(true).open(file_name).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        file.set_len(0).unwrap();
    }
}
