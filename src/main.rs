use std::fs;

struct Match {
    file_path: String,
    line_index: usize,
    line_content: String,
}

fn main() {
    let matches = check_file("./test-files/test-one.txt", "test");
    print_matches(matches)
}

fn check_file(file_path: &str, pattern: &str) -> Vec<Match> {
    let file_content = match fs::read_to_string(file_path) {
        Ok(file_content) => file_content,
        Err(_) => {
            eprintln!("Was not able to read file {file_path}");
            return Vec::new()
        }
    };


    let lines = file_content.split("\n");
    let mut matching_lines: Vec<Match> = Vec::new();

    // TODO we might be able to stream this for better memory management
    for (line_index, line_content) in lines.enumerate() {
        if line_content.contains(pattern) {
            let match_found = Match { line_index: line_index + 1, line_content: line_content.parse().unwrap(), file_path: file_path.parse().unwrap() };
            matching_lines.push(match_found);
        }
    }

    matching_lines
}

fn print_matches(matches: Vec<Match>) {
    for match_current in matches {
        println!("{}:{} {}", match_current.file_path, match_current.line_index, match_current.line_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_file_positive() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        fs::write(&file_path, "hello test 123").unwrap();

        let result = check_file(file_path.into_string().unwrap().as_str(), "test");

        assert_eq!(result.len(), 1);
        assert_eq!(result.get(0).unwrap().line_index, 1);
    }

    #[test]
    fn check_file_negative() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        fs::write(&file_path, "hello test 123").unwrap();

        let result = check_file(file_path.into_string().unwrap().as_str(), "not in there");

        assert_eq!(result.len(), 0)
    }

    #[test]
    fn check_file_many_lines() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

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