pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

pub fn insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    fn content() -> &'static str {
        "Rust:
safe, fast, productive
choose three"
    }

    #[test]
    fn case_sensitive() {
        assert_eq!(vec!["safe, fast, productive"], search("duct", content()))
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(vec!["Rust:"], insensitive_search("rust", content()))
    }
}
