use rand::{Rng, thread_rng};

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}


impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],  // | 0 0 0 0 | 4x4 matrix
        }
    }


    pub fn random_fnc(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();

        let mut res = Matrix::zeros(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0 // (0-1) X 2 == (0-2)-1 == -1-1;
            }
        }

        res
    }

    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data,
        }
    }

    pub fn multiply(&mut self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Attempted to multiply by matrix of incorrect dimensions");
        }
        let mut res = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                res.data[i][j] = sum;
            }
        }

        res
    }

    pub fn add(&mut self, other: &Matrix) -> Matrix {
        if self.cols != other.rows || self.cols != other.cols {
            panic!("Attempted to Add matrix of incorrect dimensions");
        }
        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }


        res
    }

    pub fn dot_matrix(
        &mut self, other: &Matrix,
    ) -> Matrix {
        if self.cols != other.rows || self.cols != other.cols {
            panic!("Attempted to dot multiply by matrix of incorrect dimensions");
        }
        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }

        res
    }

    pub fn subtract(&self, other: &Matrix) -> Matrix {
        if self.cols != other.rows || self.cols != other.cols {
            panic!("Attempted to Subtract matrix of incorrect dimensions");
        }
        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        res
    }

    pub fn map(&mut self, func: &dyn Fn(f64) -> f64) -> Matrix {
        Matrix::from((self.data).clone()
        .into_iter().map(|row| row.into_iter()
        .map(|value| func(value)).collect())
        .collect())
    }

    pub fn transpose(&mut self) -> Matrix {
        let mut res = Matrix::zeros(self.rows, self.cols); 
       for i in 0..self.rows {
          for j in 0..self.cols {
            res.data[i][j] = self.data[i][j];
          }
       }


       res

    }
}