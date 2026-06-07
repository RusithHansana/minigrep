pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "/
Rust:
safe, fast, productive
Pick three.
Duct tape";

        let result = search(&query, &contents);

        assert_eq!(vec!["safe, fast, productive"], result);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fase, productive.
Pick three.
Trust me.";

        let results = search_case_insensitive(&query, &contents);
        assert_eq!(vec!["Rust:", "Trust me."], results);
    }
}
