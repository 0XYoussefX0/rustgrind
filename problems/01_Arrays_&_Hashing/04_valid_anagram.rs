fn is_anagram(str1: String, str2: String) -> bool {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Case_1() {
        let str1 = String::new("racecar");
        let str2 = String::new("carrace");

        assert_eq!(is_anagram(str1, str2), true);
    }

    #[test]
    fn Case_2() {
        let str1 = String::new("jar");
        let str2 = String::new("jam");

        assert_eq!(is_anagram(str1, str2), false);
    }
}
