use regex::Regex;

pub fn valid_email(email: &str) -> bool {
    let regex = Regex::new(r"^[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+$").unwrap();

    regex.is_match(email)
}

pub fn has_number(haystack: &str) -> bool {
    let regex = Regex::new(r"^.*(\d).*$").unwrap();

    regex.is_match(haystack)
}

pub fn has_symbol(haystack: &str) -> bool {
    let regex = Regex::new(r"^.*[!@#$%^&*()_+?/:;\[\]{}|<>.,].*$").unwrap();

    regex.is_match(haystack)
}