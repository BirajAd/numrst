pub struct Library {
    books: Vec<Book>,
}

pub struct Book {
    title: String,
    year: u16,
}

impl Book {
    pub fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

impl Library {
    pub fn new() -> Library {
        Library {
            books: Vec::new(),
        }
    }

    pub fn len(self) -> usize {
        self.books.len()
    }

    pub fn is_empty(self) -> bool {
        self.books.len() == 0
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn list_books(self) {
        println!("Library has {} books", self.books.len());
        for book in self.books {
            println!("{0}, {1}", book.title, book.year);
        }
    }
}