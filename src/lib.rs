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

pub struct Matrix {
    value: Vec<Vec<i32>>,
}

impl Matrix {
    pub fn new(value: &Vec<Vec<i32>>) -> Matrix {
        Matrix {
           value: value.to_vec()
        }
    }

    pub fn empty() -> Matrix {
        Matrix {
            value: vec![]
        }
    }

    pub fn zeros(&mut self, shape: (usize, usize)) {
        println!("({} {})", shape.0, shape.1);
        self.value = vec![vec![0; shape.1]; shape.0]
    }

    pub fn randoms(&mut self, shape: (usize, usize)) {
        self.value = vec![vec![0; shape.1]; shape.0];
        for row in 0..shape.0 {
            self.value[row] = (0..shape.1).map(|_| rand::thread_rng().gen_range(1..10)).collect();
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        // assuming the vector will be evenly sized
        let row = self.value.len();
        if row > 0 {
            let col: usize = self.value[0].len();
            return (row, col);
        }
        return (0, 0);
    }

    pub fn transpose_vec(&self) -> (bool, Matrix) {
        let row = self.value.len();
        if row == 0 {
            return (true, Matrix::new(&self.value));
        } else {
            let col = self.value[0].len();
            for vec in &self.value {
                if vec.len() != col {
                    return (false, Matrix::new(&self.value));
                }
            }
            let mut ans: Vec<Vec<i32>> = vec![vec![0; row]; col];
            for i in 0..row {
                for j in 0..col {
                    if j < row && i < col {
                        ans[i][j] = self.value[j][i];
                        ans[j][i] = self.value[i][j];
                    }
                    if j >= col {
                        ans[i][j] = self.value[j][i];
                    }
                    if j >= row {
                        ans[j][i] = self.value[i][j]
                    }
                    if i >= row {
                        ans[j][i] = self.value[i][j];
                    }
                    if i >= col {
                        ans[j][i] = self.value[i][j];
                    }
                }
            }
            return (true, Matrix::new(&ans));
        }
    }

    pub fn print_vec(&self) {
        if self.value.len() == 0 {
            println!("Vector is empty");
        } else {
            let rows = self.value.len();
            let first = self.value[0].len();
            let mut good = true;
            for i in 0..rows {
                if self.value[i].len() != first {
                    println!("Vector is not evenly sized");
                    good = false;
                    break;
                }
            }
            if good {
                println!("Vector of size: ({}, {})", self.value.len(), self.value[0].len());   
                for vect in &self.value {
                    print!("|");
                    for val in vect {
                        print!(" {}", val);
                    }
                    println!(" |");
                }
            }
        }
    }
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