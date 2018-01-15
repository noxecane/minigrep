pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
