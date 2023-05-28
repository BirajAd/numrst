// use std::io;

// use start_rust::guessing_game;
use start_rust::Library;
use start_rust::Book;

fn main() {
   let mut books = vec![10, 20];
   books.push(30);
   let midpoint = books.len() / 2;
   println!("Mid value is: {}", books[midpoint]);
   for book in &books {
        println!("book: {book}");
   }
   // guessing_game();
   let mut library = Library::new();
   library.add_book(Book::new("My book", 2030));
   library.add_book(Book::new("Your book", 2020));
   // println!("{}", library.is_empty());
   library.list_books();

}
