use std::thread;
use std::time;

fn main() {
    let book = book_stats::get_book();

    let now = time::Instant::now();
    (0..5).for_each(|_| {
        book_stats::analyse_book(&book);
    });
    let after = time::Instant::now();

    println!("Sync: {}", (after - now).as_secs_f64());

    let now = time::Instant::now();
    (0..5).for_each(|_| {
        book_stats::analyse_book_threaded(&book);
    });
    let after = time::Instant::now();

    println!("Single threaded: {}", (after - now).as_secs_f64());

    let book_ref = &book;
    let now = time::Instant::now();
    thread::scope(|s| {
        (0..500).for_each(|_| {
            s.spawn(move || {
                book_stats::analyse_book_threaded(book_ref);
            });
        });
    });
    let after = time::Instant::now();

    println!("Double threaded: {}", (after - now).as_secs_f64());
}
