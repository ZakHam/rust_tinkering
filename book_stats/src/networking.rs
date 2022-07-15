use anyhow::{anyhow, Result};

pub fn get_book(url: &str) -> Result<String> {
    let book_response = reqwest::blocking::get(url)?;

    if book_response.status() != 200 {
        Err(anyhow!(
            "Error getting book from url: {}, with error code: {}",
            url,
            book_response.status()
        ))
    } else {
        Ok(book_response.text()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_book() {
        let _m = mockito::mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("a book")
            .create();

        let book = get_book(&mockito::server_url());
        assert!(book.is_ok());
        assert_eq!(book.unwrap(), "a book");
    }

    #[test]
    fn test_get_book_fails_gracefully() {
        let _m = mockito::mock("GET", "/").with_status(500).create();

        let book = get_book(&mockito::server_url());
        assert!(book.is_err());
    }
}
