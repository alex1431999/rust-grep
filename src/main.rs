use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::JoinHandle;
use std::{env, fs, thread};

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

    let matches = check_dir(dir_path, pattern, true);
    print_matches(matches, pattern)
}

fn check_dir(dir_path: &str, pattern: &str, use_threading: bool) -> Vec<Match> {
    let mut matches: Vec<Match> = Vec::new();

    let dir = match fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("Was not able to read dir {dir_path}");
            return matches;
        }
    };

    let mut threads: Vec<JoinHandle<Vec<Match>>> = Vec::new();
    for dir_entry in dir {
        let dir_entry_path = dir_entry.unwrap().path();

        if dir_entry_path.is_dir() {
            let mut matches_in_sub_dir =
                check_dir(dir_entry_path.to_str().unwrap(), pattern, use_threading);
            matches.append(&mut matches_in_sub_dir);
        } else {
            let pattern_copy = pattern.to_owned();
            let dir_entry_path_copy = dir_entry_path.to_str().unwrap().to_owned();

            if use_threading {
                let handle = thread::spawn(move || {
                    return check_file(&dir_entry_path_copy, &pattern_copy);
                });

                threads.push(handle);
            } else {
                let mut matches_to_add = check_file(&dir_entry_path_copy, &pattern_copy);
                matches.append(&mut matches_to_add);
            }
        }
    }

    if use_threading {
        for thread in threads {
            let mut matches_to_add = thread.join().unwrap();
            matches.append(&mut matches_to_add);
        }
    }
    matches
}

fn check_file(file_path: &str, pattern: &str) -> Vec<Match> {
    let mut matches: Vec<Match> = Vec::new();

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Was not able to read file {file_path}");
            return matches;
        }
    };

    let reader = BufReader::new(file);

    for (line_index, line_content) in reader.lines().enumerate() {
        if let Ok(line_content_string) = line_content
            && line_content_string.contains(pattern)
        {
            let match_found = Match {
                line_index: line_index + 1,
                line_content: line_content_string,
                file_path: file_path.to_owned(),
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
    use std::path::PathBuf;
    use std::time::Instant;

    fn create_large_file(file_path: &PathBuf, amount_of_lines: usize) {
        let line_text = "hello test 123\n";
        let file_content = line_text.repeat(amount_of_lines);

        fs::write(&file_path, file_content).unwrap();
    }

    #[test]
    fn check_dir_basic() {
        let dir = tempfile::tempdir().unwrap();
        let file_path_first_file = dir.path().join("test_first.txt");
        let file_path_second_file = dir.path().join("test_second.txt");

        fs::write(&file_path_first_file, "hello test 123").unwrap();
        fs::write(&file_path_second_file, "hello test 123").unwrap();

        let result = check_dir(dir.path().to_str().unwrap(), "test", true);

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

        create_large_file(&file_path, 100000);

        let result = check_file(file_path.into_string().unwrap().as_str(), "test");

        assert_eq!(result.len(), 100000)
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

    #[test]
    fn performance() {
        let start = Instant::now();
        let dir_amount = 10;
        let files_per_dir_amount = 100;

        let base_dir = tempfile::tempdir().unwrap();

        for i in 0..dir_amount {
            fs::create_dir(base_dir.path().join(format!("dir_{i}"))).unwrap();

            for j in 0..files_per_dir_amount {
                let file_path = base_dir.path().join(format!("dir_{i}/file_{j}"));
                create_large_file(&file_path, 5000)
            }
        }

        check_dir(base_dir.path().to_str().unwrap(), "test", true);

        let elapsed = start.elapsed();

        println!("Elapsed: {:?}", elapsed)
    }
}
