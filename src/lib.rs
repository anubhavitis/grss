pub fn print_matches(keyword: &String, content: &String) {
    for line in content.lines() {
        if line.contains(keyword) {
            println!("{}", line);
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
