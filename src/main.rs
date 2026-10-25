use std::fs;

fn main() {
    let result = check_file("./test-files/test-one.txt", "test2");

    println!("Result {}", result)
}

fn check_file(path: &str, pattern: &str) -> bool {
    let file_content = fs::read_to_string(path);
    let binding = file_content.unwrap();
    let lines = binding.split("\n");

    for line in lines {
        if line.contains(pattern) {
            return true
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;
    use super::*;

    #[test]
    fn check_file_positive() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("test.txt");

        fs::write(&file_path, "hello test 123").unwrap();

        let result = check_file(file_path.into_string().unwrap().as_str(), "test");

        assert!(result)
    }

    #[test]
    fn check_file_negative() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("test.txt");

        fs::write(&file_path, "hello test 123").unwrap();

        let result = check_file(file_path.into_string().unwrap().as_str(), "not in there");

        assert!(!result)
    }
}