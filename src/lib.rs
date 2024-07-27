use std::io::{self, Write};

pub fn print_matches(keyword: &String, content: &String) {
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout);
    for line in content.lines() {
        if line.contains(keyword) {
            writeln!(handle, "{}", line).expect("failed to write response");
        }
    }
}

pub fn count_matches(keyword: &String, content: &String) {
    let mut count = 0;
    for line in content.lines() {
        if line.contains(keyword) {
            count += 1;
        }
    }

    println!("{} lines found with keyword: {}", count, keyword);
}
