// use std::io;

// use start_rust::guessing_game;
// use start_rust::Library;
// use start_rust::Book;
use start_rust::Matrix;
use rand::Rng;

fn print_vec(vector: &Vec<Vec<i32>>) {
   if vector.len() == 0 {
      println!("Vector is empty");
   } else {
      let rows = vector.len();
      let first = vector[0].len();
      let mut good = true;
      for i in 0..rows {
         if vector[i].len() != first {
            println!("Vector is not evenly sized");
            good = false;
            break;
         }
      }
      if good {
         println!("Vector of size: ({}, {})", vector.len(), vector[0].len());   
         for vect in vector {
            print!("|");
            for val in vect {
               print!(" {}", val);
            }
            println!(" |");
         }
      }
   }
}

fn main() {
   // let mut books: Vec<i32> = vec![10, 20];
   let m1: Vec<Vec<i32>> = vec![
      vec![ 1, 2, 3, 4 ],
      vec![ 4, 5, 6, 5 ],
      vec![ 7, 8, 9, 7 ],
      vec![ 2, 3, 8, 6 ],
      vec![ 7, 0, 1, 2 ]
   ];
   // let mtrx = Matrix::new(&m1);
   // mtrx.print_vec();
   // let (_st, transposed) = mtrx.transpose_vec();
   // transposed.print_vec();

   let mut a_mtrx = Matrix::empty();
   a_mtrx.randoms((7, 2000000));
   // a_mtrx.zeros((7,2000000));
   a_mtrx.print_vec();
   // let m2: Vec<Vec<i32>> = vec![vec![0; 3]; 4];
   // print_vec(&m1);
   // let (status, res) = transpose_vec(&m1);
   // if status {
      // print_vec(&res);
   // }

   // books.push(30);
   // let midpoint = books.len() / 2;
   // println!("Mid value is: {}, size is: {}", books[midpoint], books.len());
   // for book in &books {
      // println!("book: {book}");
   // }
   // guessing_game();
   // let mut library = Library::new();
   // library.add_book(Book::new("My book", 2030));
   // library.add_book(Book::new("Your book", 2020));
   // println!("{}", library.is_empty());
   // library.list_books();
}
