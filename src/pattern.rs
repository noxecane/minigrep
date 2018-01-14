pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
choose three";
        assert_eq!(
            vec!["safe, fast, productive"],
            search(query, contents)
        )
    }
}
