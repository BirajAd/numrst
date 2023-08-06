use rand::Rng;

pub struct Matrix {
    value: Vec<Vec<i32>>,
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            value: Vec::new()
        }
    }

    pub fn from(value: &Vec<Vec<i32>>) -> Matrix {
        Matrix {
           value: value.to_vec()
        }
    }

    pub fn empty() -> Matrix {
        Matrix {
            value: vec![]
        }
    }

    pub fn randoms(shape: (usize, usize)) -> Matrix {
        let mut temp = vec![vec![0; shape.1]; shape.0];
        for row in 0..shape.0 {
            temp[row] = (0..shape.1).map(|_| rand::thread_rng().gen_range(1..10)).collect();
        }
        return Matrix {
            value: temp
        };
    }

    pub fn zeros(&mut self, shape: (usize, usize)) {
        println!("({} {})", shape.0, shape.1);
        self.value = vec![vec![0; shape.1]; shape.0]
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
            return (true, Matrix::from(&self.value));
        } else {
            let col = self.value[0].len();
            for vec in &self.value {
                if vec.len() != col {
                    return (false, Matrix::from(&self.value));
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
            return (true, Matrix::from(&ans));
        }
    }

    fn print_single_vec(&self, vect: &Vec<i32>) {
        let length = vect.len();
        print!("|");
        if length > 10 {
            for ind in 0..5 {
                print!(" {}", vect[ind]);
            }
            print!(", ... ,");
            for ind in length-5..length {
                print!(" {}", vect[ind]);
            }
        } else {
            for val in vect {
                print!(" {}", val);
            }
        }
        println!(" |");
    }

    pub fn print_vec(&self) {
        if self.value.len() == 0 {
            println!("Vector is empty");
        } else {
            // let rows = self.value.len();
            // let first = self.value[0].len();
            let good = true;
            // for i in 0..rows {
                // if self.value[i].len() != first {
                    // println!("Vector is not evenly sized");
                    // good = false;
                    // break;
                // }
            // }
            if good {
                let rows: usize = self.value.len();
                let columns: usize = self.value[0].len();
                println!("Vector of size: ({}, {})", rows, columns);   
                if rows < 7 {
                    for vect in &self.value {
                        self.print_single_vec(vect);
                    }
                } else {
                    for ind in 0..3 {
                        self.print_single_vec(&self.value[ind]);
                    }
                    print!("...,\n");
                    for ind in rows-3..rows {
                        self.print_single_vec(&self.value[ind]);
                    }
                }
            }
        }
    }
}
