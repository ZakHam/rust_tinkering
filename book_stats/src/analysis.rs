use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn get_word_count(book: &str) -> usize {
    let word_re = regex::Regex::new(r"(\S)+").unwrap();
    word_re.find_iter(book).count()
}

pub fn get_line_count(book: &str) -> usize {
    let line_re = regex::Regex::new(r"(\S)+\r\n").unwrap();
    line_re.find_iter(book).count()
}

pub fn get_most_common_word(book: &str) -> (String, usize) {
    let mut words_map: HashMap<String, usize> = HashMap::new();
    let word_re = regex::Regex::new(r"(\S)+").unwrap();
    word_re.find_iter(book).for_each(|m| {
        let lowercase_word = m.as_str().to_lowercase();
        match words_map.entry(lowercase_word) {
            Entry::Occupied(mut e) => e.insert(e.get() + 1),
            Entry::Vacant(e) => *e.insert(1),
        };
    });

    let (word, count) = words_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    (word.to_string(), *count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_BOOK: &str =
        "this is A t#st boOk\r\nWith more th&n one Line\r\n and sp3ci@l characters\r\n and the word and 3 times\r\n";

    #[test]
    fn test_count_words() {
        let word_count = get_word_count(TEST_BOOK);
        assert_eq!(word_count, 19);
    }

    #[test]
    fn test_count_lines() {
        let line_count = get_line_count(TEST_BOOK);
        assert_eq!(line_count, 4);
    }

    #[test]
    fn test_get_most_common_word() {
        let most_common_word = get_most_common_word(TEST_BOOK);
        assert_eq!(most_common_word, (String::from("and"), 3usize));
    }
}
