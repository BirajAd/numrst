// use std::io;

// use start_rust::guessing_game;
// use start_rust::Library;
// use start_rust::Book;
mod matrix;
// mod library;
use matrix::Matrix;
// use library::{ Library, Book };
// use rand::Rng;
// use std::env;

// #[warn(dead_code)]
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
   use std::time::Instant;
   // let mut books: Vec<i32> = vec![10, 20];
   let _m1: Vec<Vec<f32>> = vec![
      vec![ 1.0, 2.0, 3.0, 4.0 ],
      vec![ 4.0, 5.0, 6.0, 5.0 ],
      vec![ 7.0, 8.0, 9.0, 7.0 ],
      vec![ 2.0, 3.0, 8.0, 6.0 ],
      vec![ 7.1, 0.0, 1.0, 2.0 ]
   ];
   // let args: Vec<String> = env::args().collect();

   let now = Instant::now();
   let a_mtrx = Matrix::randoms((7,20_000_000));
   let elapsed = now.elapsed();
   println!("Elapsed: {:.2?}", elapsed);
   // let text = String::from("my name is biraj adhikari.");
   // for word in text.split_whitespace() {
      // println!("{word}");
   // }
   // a_mtrx.transpose_vec();
   // a_mtrx.zeros((7,2000000));
   a_mtrx.print_vec();
   // let m2: Vec<Vec<i32>> = vec![vec![0; 3]; 4];
   // print_vec(&m1);

   // guessing_game();
   // let mut library = Library::new();
   // library.add_book(Book::new("My book", 2030));
   // library.add_book(Book::new("Your book", 2020));
   // println!("{}", library.is_empty());
   // library.list_books();
}
