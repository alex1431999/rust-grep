use std::fs;

fn main() {
    let result = check_file("./test-files/test-one.txt", "test");

    println!("Result {:?}", result)
}

fn check_file(file_path: &str, pattern: &str) -> Vec<usize> {
    let file_content = fs::read_to_string(file_path);
    let binding = file_content.unwrap();
    let lines = binding.split("\n");
    let mut matching_lines: Vec<usize> = Vec::new();

    for (line_index, line_content) in lines.enumerate() {
        if line_content.contains(pattern) {
            matching_lines.push(line_index)
        }
    }

    matching_lines
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

        assert_eq!(result.len(), 1);
        assert_eq!(*result.get(0).unwrap(), 0);
    }

    #[test]
    fn check_file_negative() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("test.txt");

        fs::write(&file_path, "hello test 123").unwrap();

        let result = check_file(file_path.into_string().unwrap().as_str(), "not in there");

        assert_eq!(result.len(), 0)
    }
}