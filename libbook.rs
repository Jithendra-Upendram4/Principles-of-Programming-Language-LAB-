#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    is_issued: bool,
}

impl Book {
    fn new(title: &str, author: &str, isbn: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            isbn: isbn.to_string(),
            is_issued: false,
        }
    }

    fn print_details(&self) {
        println!(
            "Title: {}, Author: {}, ISBN: {}, Issued: {}",
            self.title, self.author, self.isbn, self.is_issued  // Fixed this line
        );
    }
}

fn issue_book(book: Book) -> Book {
    let mut issued_book = book;
    issued_book.is_issued = true;
    issued_book.print_details();  // Reading fields to avoid dead code warning
    issued_book
}

fn main() {
    let book1 = Book::new("The Rust Programming Language", "Steve Klabnik", "12345");
    let backup = book1.clone(); // Keeping a backup

    let issued_book = issue_book(book1);

    println!("Issued Book: {:?}", issued_book);
    println!("Backup Book: {:?}", backup); // Backup is still accessible
}

