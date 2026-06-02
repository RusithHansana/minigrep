pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "/
        Rust:
        safe, fast, productive
        Pick three.";

        let result = search(&query, &contents);

        assert_eq!(vec!["safe, fast, productive"], result);
    }
}
