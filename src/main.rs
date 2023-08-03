// use std::io;

// use start_rust::guessing_game;
// use start_rust::Library;
// use start_rust::Book;
use start_rust::Matrix;

fn pretty_print(matrix: &[[i32; 3]; 3]) {
   for row in matrix {
      print!("|");
      for val in row {
         print!(" {}", val);
      }
      println!(" |")
   }
}

fn transpose(mut matrix: [[i32; 3]; 3]) {
   for i in 0..3 {
      for j in i..3 {
         let temp: i32 = matrix[i][j];
         matrix[i][j] = matrix[j][i];
         matrix[j][i] = temp;
      }
   }
   pretty_print(&matrix);
}

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

fn transpose_vec(vector: &Vec<Vec<i32>>) -> (bool, Vec<Vec<i32>>) {
   let row = vector.len();
   if row == 0 {
      return (true, vector.to_vec());
   } else {
      let col = vector[0].len();
      for vec in vector {
         if vec.len() != col {
            return (false, vector.to_vec())
         }
      }
      let mut ans: Vec<Vec<i32>> = vec![vec![0; row]; col];
      for i in 0..row {
         for j in 0..col {
            if j < row && i < col {
               ans[i][j] = vector[j][i];
               ans[j][i] = vector[i][j];
            }
            if j >= col {
               ans[i][j] = vector[j][i];
            }
            if j >= row {
               ans[j][i] = vector[i][j]
            }
            if i >= row {
               ans[j][i] = vector[i][j];
            }
            if i >= col {
               ans[j][i] = vector[i][j];
            }
         }
      }
      return (true, ans);
   }
}


fn main() {
   // let mut books: Vec<i32> = vec![10, 20];
   let m1: Vec<Vec<i32>> = vec![
      vec![ 1, 2, 3, 4 ],
      vec![ 4, 5, 6, 5 ],
      vec![ 7, 8, 9, 7 ],
      vec![ 2, 3, 8, 6 ]
   ];
   let mtrx = Matrix::new(&m1);
   mtrx.print_vec();
   let (st, transposed) = mtrx.transpose_vec();
   transposed.print_vec();
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
   // let matrix = [
      // [101, 102, 103],
      // [201, 202, 203],
      // [301, 302, 303]
   // ];

   // pretty_print(&matrix);
   // println!("===================");
   // transpose(matrix);
}
