use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn guessing_game() {
    println!("Guess a number between 1 and 100: ");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to take input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        println!("Your guess: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You got it right");
                break;
            }
        }
    }

    println!("The secret number is: {secret_number}");
}

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
        for book in self.books {
            println!("{0}, {1}", book.title, book.year);
        }
    }
}