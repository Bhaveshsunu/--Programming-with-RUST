#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    is_issued: bool,
}

impl Book {
    // Function to issue the book without transferring ownership
    fn issue_book(&mut self) {
        if !self.is_issued {
            self.is_issued = true;
            println!(
                "The book '{}' by {} (ISBN: {}) has been issued.",
                self.title, self.author, self.isbn
            );
        } else {
            println!("This book '{}' has already been issued!", self.title);
        }
    }

    // Function to return the book
    fn return_book(&mut self) {
        if self.is_issued {
            self.is_issued = false;
            println!(
                "The book '{}' by {} (ISBN: {}) has been returned.",
                self.title, self.author, self.isbn
            );
        } else {
            println!("This book '{}' was not issued!", self.title);
        }
    }
}

fn main() {
    // Create a new book
    let mut book = Book {
        title: String::from("Rust Programming"),
        author: String::from("John Doe"),
        isbn: String::from("1234567890"),
        is_issued: false,
    };

    // Display initial book state
    println!(
        "Initial Book Details:\nTitle: {}\nAuthor: {}\nISBN: {}\nIssued: {}",
        book.title, book.author, book.isbn, book.is_issued
    );

    // Issue the book
    book.issue_book();
    println!(
        "After issuing:\nTitle: {}\nAuthor: {}\nISBN: {}\nIssued: {}",
        book.title, book.author, book.isbn, book.is_issued
    );

    // Return the book
    book.return_book();
    println!(
        "After returning:\nTitle: {}\nAuthor: {}\nISBN: {}\nIssued: {}",
        book.title, book.author, book.isbn, book.is_issued
    );
}

