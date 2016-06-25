use std::ops::{Add, Sub, Mul};

/// Generic matrix with fixed sized rows and columns
pub struct Matrix<const rows: usize, const cols: usize, T> {
    inner: [T; rows * cols]
}

/// Multiply two matrices
///
/// Given matrices `A` and `B`, `A ⨉ B = C` is defined when `columns(A) == rows(B)`.
/// `C` will be of size `(rows(A), columns(B))`.
impl<
    const lhs_rows: usize, const lhs_cols: usize,
    const rhs_rows: usize, const rhs_cols: usize,
    T,
> Mul<Matrix<rhs_rows, rhs_cols, T>> for Matrix<lhs_rows, lhs_cols, T> where
    lhs_cols == rhs_rows
{
    type Output = Matrix<lhs_rows, rhs_cols, T>;
    
    fn mul(self, _rhs: Matrix<rhs_rows, rhs_cols, T>) -> Self::Output {
        unimplemented!()
    }
}

/// Multiply matrix with scalar
///
/// Results in matrix of same size
impl<const lhs_rows: usize, const lhs_cols: usize, T> Mul<T> for Matrix<lhs_rows, lhs_cols, T> where
    T: Mul<RHS=T>
{
    type Output = Matrix<lhs_rows, lhs_cols, T>;
    
    fn mul(self, _rhs: T) -> Self::Output {
        unimplemented!()
    }
}

/// In linear algebra, the determinant is a useful value that can be computed from the elements of a square matrix.
///
/// More info on [Wikipedia](https://en.wikipedia.org/wiki/Determinant).
trait Determinant {
    type Output;

    fn determinant(&self) -> Self::Output;
}

/// Implement determinant for square matrices
impl<const lhs_rows: usize, const lhs_cols: usize, T, I> Determinant for Matrix<lhs_rows, lhs_cols, T> where
    /// square matrix
    lhs_rows == lhs_cols,
    /// Multiply one element with the determinant of a subset of the matrix
    T: Mul<RHS=<Matrix<lhs_rows - 1, lhs_cols - 1, T> as Determinant>::Output>,
    /// Add/substract the outputs of these multiplications
    I: Add<RHS=<T as Mul<RHS=<Matrix<lhs_rows - 1, lhs_cols - 1, T> as Determinant>::Output>>::Output>,
    I: Sub<RHS=<T as Mul<RHS=<Matrix<lhs_rows - 1, lhs_cols - 1, T> as Determinant>::Output>>::Output>,
{
    type Output = I;
    
    fn determinant(&self) -> Self::Output {
        unimplemented!()
    }
}

