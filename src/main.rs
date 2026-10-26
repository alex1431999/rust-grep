use std::{env, fs};

struct Match {
    file_path: String,
    line_index: usize,
    line_content: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let pattern = match args.get(1) {
        Some(pattern) => pattern,
        None => panic!("You need to supply a pattern"),
    };

    let dir_path = match args.get(2) {
        Some(dir_path) => dir_path,
        None => "./test-files",
    };

    let matches = check_dir(dir_path, pattern);
    print_matches(matches, pattern)
}

fn check_dir(dir_path: &str, pattern: &str) -> Vec<Match> {
    let mut matches: Vec<Match> = Vec::new();

    let dir = match fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("Was not able to read dir {dir_path}");
            return matches;
        }
    };

    for dir_entry in dir {
        let dir_entry_path = dir_entry.unwrap().path();

        if dir_entry_path.is_dir() {
            let mut matches_in_sub_dir = check_dir(dir_entry_path.to_str().unwrap(), pattern);
            matches.append(&mut matches_in_sub_dir);
        } else {
            let mut matches_in_file = check_file(dir_entry_path.to_str().unwrap(), pattern);
            matches.append(&mut matches_in_file)
        }
    }

    matches
}

fn check_file(file_path: &str, pattern: &str) -> Vec<Match> {
    let mut matches: Vec<Match> = Vec::new();

    let file_content = match fs::read_to_string(file_path) {
        Ok(file_content) => file_content,
        Err(_) => {
            eprintln!("Was not able to read file {file_path}");
            return matches;
        }
    };

    let lines = file_content.split("\n");

    // TODO we might be able to stream this for better memory management
    for (line_index, line_content) in lines.enumerate() {
        if line_content.contains(pattern) {
            let match_found = Match {
                line_index: line_index + 1,
                line_content: line_content.parse().unwrap(),
                file_path: file_path.parse().unwrap(),
            };
            matches.push(match_found);
        }
    }

    matches
}

fn print_matches(matches: Vec<Match>, pattern: &str) {
    for match_current in matches {
        println!(
            "{}:{} {}",
            match_current.file_path,
            match_current.line_index,
            highlight_pattern_in_content(match_current.line_content.as_str(), pattern)
        )
    }
}

fn highlight_pattern_in_content(content: &str, pattern: &str) -> String {
    let parts: Vec<&str> = content.split(pattern).collect();
    parts.join(format!("\x1b[43m {pattern} \x1b[0m").as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_dir_basic() {
        let dir = tempfile::tempdir().unwrap();
        let file_path_first_file = dir.path().join("test_first.txt");
        let file_path_second_file = dir.path().join("test_second.txt");

        fs::write(&file_path_first_file, "hello test 123").unwrap();
        fs::write(&file_path_second_file, "hello test 123").unwrap();

        let result = check_dir(dir.path().to_str().unwrap(), "test");

        assert_eq!(result.len(), 2);
        assert_eq!(result.get(0).unwrap().line_index, 1);
    }

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

    #[test]
    fn highlight_pattern_in_content_basic() {
        assert_eq!(
            highlight_pattern_in_content("test hello there", "hello"),
            "test \x1b[43m hello \x1b[0m there"
        )
    }
}
