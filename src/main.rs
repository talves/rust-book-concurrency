// use rust_book_concurrency;

fn main() {
    rust_book_concurrency::spawn_threads(true);
    rust_book_concurrency::spawn_threads(false);

    rust_book_concurrency::move_closures_threads();

    rust_book_concurrency::message_threads();
    rust_book_concurrency::multi_message_threads();
    rust_book_concurrency::multi_producers_threads();
}
