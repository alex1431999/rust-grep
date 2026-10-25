use std::fs;

fn main() {
    let result = check_file("./test-files/test-one.txt", "test");

    println!("Result {:?}", result)
}

fn check_file(file_path: &str, pattern: &str) -> Vec<usize> {
    let file_content = match fs::read_to_string(file_path) {
        Ok(file_content) => file_content,
        Err(_) => {
            eprintln!("Was not able to read file {file_path}");
            return Vec::new()
        }
    };


    let lines = file_content.split("\n");
    let mut matching_lines: Vec<usize> = Vec::new();

    // TODO we might be able to stream this for better memory management
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

    #[test]
    fn check_file_many_lines() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("test.txt");
        let amount_of_lines = 100000;
        let line_text = "hello test 123\n";
        let file_content = line_text.repeat(amount_of_lines);

        fs::write(&file_path, file_content).unwrap();

        let result = check_file(file_path.into_string().unwrap().as_str(), "test");

        assert_eq!(result.len(), amount_of_lines)
    }

    #[test]
    fn check_file_handles_file_it_cant_read() {
        let result = check_file("does_not_exist.txt", "test");
        assert_eq!(result.len(), 0)
    }
}