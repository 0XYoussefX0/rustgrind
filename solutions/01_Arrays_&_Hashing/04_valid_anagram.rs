fn is_anagram(str1: String, str2: String) -> bool {
    if str1.len() != str2.len() {
        return false;
    }

    let mut str1_char_frequency: HashMap<char, u32> = HashMap::new();
    let mut str2_char_frequency: HashMap<char, u32> = HashMap::new();

    for (index, char) in str1.chars().enumerate() {
        let str1_char = char;
        let str2_char = str2.chars().nth(index).unwrap();

        if str1_char_frequency.contains_key(&str1_char) {
            let current_count = str1_char_frequency.get(&str1_char).unwrap();

            str1_char_frequency.insert(str1_char, current_count + 1);
        } else {
            str1_char_frequency.insert(str1_char, 0);
        }

        if str2_char_frequency.contains_key(&str2_char) {
            let current_count = str2_char_frequency.get(&str2_char).unwrap();

            str2_char_frequency.insert(str2_char, current_count + 1);
        } else {
            str2_char_frequency.insert(str2_char, 0);
        }
    }

    for (char, frequency) in str2_char_frequency {
        let str1_char_entry = str1_char_frequency.get(&char);

        match str1_char_entry {
            Some(str1_char_frequency) => {
                if frequency != *str1_char_frequency {
                    return false;
                }
            }
            None => return false,
        }
    }

    true
}
