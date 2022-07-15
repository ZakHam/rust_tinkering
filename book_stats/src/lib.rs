use std::fmt;
use std::sync;
use std::thread;

mod analysis;
mod filesystem;
mod networking;

const TEST_BOOK: &str = "sherlock_holmes.txt";

#[derive(Debug, Default)]
pub struct BookStats {
    name: String,
    word_count: usize,
    line_count: usize,
    most_common_word: (String, usize),
}

impl fmt::Display for BookStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} stats:\nWord count: {}, line count: {}, most common word: {} ({})",
            self.name,
            self.word_count,
            self.line_count,
            self.most_common_word.0,
            self.most_common_word.1
        )
    }
}

pub fn get_book() -> String {
    if filesystem::book_exists(TEST_BOOK) {
        filesystem::read_book_from_disk(TEST_BOOK)
    } else {
        let book = networking::get_book("https://www.gutenberg.org/files/1661/1661-0.txt")
            .unwrap_or_else(|e| panic!("Error getting book to analyse: {}", e));

        filesystem::write_book_to_disk(TEST_BOOK, &book);
        book
    }
}

pub fn analyse_book(book: &str) -> BookStats {
    let word_count = analysis::get_word_count(book);
    let line_count = analysis::get_line_count(book);
    let most_common_word = analysis::get_most_common_word(book);

    BookStats {
        name: String::from(TEST_BOOK.strip_suffix(".txt").unwrap()),
        word_count,
        line_count,
        most_common_word,
    }
}

pub fn analyse_book_threaded(book: &str) -> BookStats {
    let (wc_tx, wc_rx) = sync::mpsc::channel();
    let (lc_tx, lc_rx) = sync::mpsc::channel();
    let (mcw_tx, mcw_rx) = sync::mpsc::channel();

    thread::scope(|s| {
        s.spawn(move || {
            wc_tx.send(analysis::get_word_count(book)).unwrap();
        });
        s.spawn(move || {
            lc_tx.send(analysis::get_line_count(book)).unwrap();
        });
        s.spawn(move || {
            mcw_tx.send(analysis::get_most_common_word(book)).unwrap();
        });
    });

    let word_count = wc_rx.recv().unwrap();
    let line_count = lc_rx.recv().unwrap();
    let most_common_word = mcw_rx.recv().unwrap();

    BookStats {
        name: String::from(TEST_BOOK.strip_suffix(".txt").unwrap()),
        word_count,
        line_count,
        most_common_word,
    }
}
