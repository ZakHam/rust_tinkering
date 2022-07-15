mod analysis;
mod filesystem;
mod networking;

const TEST_BOOK: &str = "sherlock_holmes.txt";

pub fn analyse_book() {
    let book = get_book();

    let word_count = analysis::get_word_count(&book);
    println!("Book has {} words", word_count);

    let line_count = analysis::get_line_count(&book);
    println!("Book has {} lines", line_count);

    let (most_common_word, count) = analysis::get_most_common_word(&book);
    println!(
        "Book has most common word: {}, occurring {} times",
        most_common_word, count
    );
}

fn get_book() -> String {
    if filesystem::book_exists(TEST_BOOK) {
        filesystem::read_book_from_disk(TEST_BOOK)
    } else {
        let book = networking::get_book("https://www.gutenberg.org/files/1661/1661-0.txt")
            .unwrap_or_else(|e| panic!("Error getting book to analyse: {}", e));

        filesystem::write_book_to_disk(TEST_BOOK, &book);
        book
    }
}
