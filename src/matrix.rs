use std::{fmt, sync::Arc};

use crossbeam::atomic::AtomicCell;

use crate::m;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f32>
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f32>) -> Matrix {
        Matrix { rows, cols, data }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.data[row * self.cols + col] = value;
    }

    pub fn identity() -> Matrix {
        m! [
            1.0, 0.0, 0.0, 0.0;
            0.0, 1.0, 0.0, 0.0;
            0.0, 0.0, 1.0, 0.0;
            0.0, 0.0, 0.0, 1.0
        ]
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Matrix {
        m! [
            1.0, 0.0, 0.0, x;
            0.0, 1.0, 0.0, y;
            0.0, 0.0, 1.0, z;
            0.0, 0.0, 0.0, 1.0
        ]
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Matrix {
        m! [
            x, 0.0, 0.0, 0.0;
            0.0, y, 0.0, 0.0;
            0.0, 0.0, z, 0.0;
            0.0, 0.0, 0.0, 1.0
        ]
    }

    pub fn rotate_x(angle: f32) -> Matrix {
        let s = angle.sin();
        let c = angle.cos();

        m! [
            1.0, 0.0, 0.0, 0.0;
            0.0, c, -s, 0.0;
            0.0, s, c, 0.0;
            0.0, 0.0, 0.0, 1.0
        ]
    }

    pub fn rotate_y(angle: f32) -> Matrix {
        let s = angle.sin();
        let c = angle.cos();

        m! [
            c, 0.0, s, 0.0;
            0.0, 1.0, 0.0, 0.0;
            -s, 0.0, c, 0.0;
            0.0, 0.0, 0.0, 1.0
        ]
    }

    pub fn rotate_z(angle: f32) -> Matrix {
        let s = angle.sin();
        let c = angle.cos();

        m! [
            c, -s, 0.0, 0.0;
            s, c, 0.0, 0.0;
            0.0, 0.0, 1.0, 0.0;
            0.0, 0.0, 0.0, 1.0
        ]
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);

        let result = Arc::new(AtomicCell::new(Matrix::new(self.rows, other.cols, vec![0.; self.rows * other.cols])));

        rayon::scope(|s| {
            for i in 0..self.rows {
                for j in 0..other.cols {
                    let result = Arc::clone(&result);
                    s.spawn(move |_| {
                        let mut sum = 0.0;

                        for k in 0..self.cols {
                            sum += self.get(i, k) * other.get(k, j);
                        }

                        unsafe {
                            (*result.as_ptr()).set(i, j, sum);
                        }
                    });
                }
            }
        });

        unsafe { (*result.as_ptr()).clone() }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{} ", self.data[i * self.cols + j])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}