use std::fs;
use std::path;

pub fn book_exists(name: &str) -> bool {
    let path = path::Path::new(&name);
    path.exists()
}

pub fn write_book_to_disk(name: &str, book: &str) {
    fs::write(name, book).unwrap();
}

pub fn read_book_from_disk(name: &str) -> String {
    fs::read_to_string(name).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    const TEST_BOOK: &str = "a book";

    #[test]
    fn test_book_exists() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(TEST_BOOK.as_ref())
            .expect("Test file should be writable");
        let file_name = file.into_temp_path();

        assert!(book_exists(file_name.to_str().unwrap()));
    }

    #[test]
    fn test_write_book() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let file_name = file.into_temp_path();
        write_book_to_disk(file_name.to_str().unwrap(), TEST_BOOK);

        let book = fs::read_to_string(file_name).unwrap();
        assert_eq!(&book, TEST_BOOK);
    }

    #[test]
    fn test_read_book() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(TEST_BOOK.as_ref())
            .expect("Test file should be writable");
        let file_name = file.into_temp_path();

        let book = read_book_from_disk(file_name.to_str().unwrap());
        assert_eq!(&book, TEST_BOOK);
    }
}
