// Define the Book struct
struct Book {
    // TODO: Add fields for book properties (title, author, year, isbn)
    title: String,
    author: String,
    year: u16,
    isbn: String,
}

impl Book {
    fn new(title: &str, author: &str, year: u16, isbn: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            isbn: isbn.to_string(),
            year,
        }
    }
}

// Define a BookStatus enum to track availability
#[derive(Debug)]
enum BookStatus {
    // TODO: Add variants for different states (Available, Borrowed)
    Available,
    Borrowed,
}

// Define a Library struct to manage books
struct Library {
    books: Vec<(Book, BookStatus)>,
}

// TODO: Implement methods for the Library struct
impl Library {
    // Create a new, empty library
    fn new() -> Library {
        // TODO: Implement this function
        let new_library = Library {
            books: Vec::new(),
        };
        new_library
    }

    // Add a book to the library
    fn add_book(&mut self, book: Book) {
        // TODO: Implement this function
        self.books.push((book, BookStatus::Available));
    }

    // Borrow a book from the library
    fn borrow_book(&mut self, isbn: &str) -> Result<&Book, &str> {
        // TODO: Implement this function
        if let Some(index) = self.books.iter().position(|(book, _)| book.isbn == isbn) {
            if matches!(self.books[index].1, BookStatus::Borrowed) {
                return Err("Can't borrow a borrowed book");
            }
            self.books[index].1 = BookStatus::Borrowed;
            return Ok(&self.books[index].0);
        }
        return Err("Book not found");
    }

    // Return a borrowed book to the library
    fn return_book(&mut self, isbn: &str) -> Result<&Book, &str> {
        // TODO: Implement this function
        if let Some(index) = self.books.iter().position(|(book, _)| book.isbn == isbn) {
            if matches!(self.books[index].1, BookStatus::Available) {
                return Err("Can't return if it's not borrowed");
            }
            self.books[index].1 = BookStatus::Available;
            return Ok(&self.books[index].0);
        }
        return Err("Book not found");
    }

    // List all books in the library with their status
    fn list_books(&self) {
        // TODO: Implement this function
        for (book, book_status) in self.books.iter() {
            println!("{} by {} ({}) - {:?}", book.title, book.author, book.year, book_status);
        }
    }
}

fn main() {
    // Create a new library
    let mut library = Library::new();
    
    // Add several books to the library
    library.add_book(Book::new(
        "The Rust Programming Language",
        "Steve Klabnik and Carol Nichols",
        2018,
        "9781718500440"
    ));
    
    library.add_book(Book::new(
        "Design Patterns",
        "Erich Gamma et al.",
        1994,
        "9780201633610"
    ));
    
    library.add_book(Book::new(
        "Clean Code",
        "Robert C. Martin",
        2008,
        "9780132350884"
    ));
    
    // List all books
    library.list_books();
    
    // Borrow a book
    println!("Borrowing \"Clean Code\"...");
    match library.borrow_book("9780132350884") {
        Ok(_) => println!("Book borrowed successfully!"),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // List all books again to see the updated status
    library.list_books();
    
    // Return the book
    println!("Returning \"Clean Code\"...");
    match library.return_book("9780132350884") {
        Ok(_) => println!("Book returned successfully!"),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // List all books one more time
    library.list_books();
}