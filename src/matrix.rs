use std::ops::Index;

/// Matrix represents a 2D matrix in a flat vector where the indexing is row major
pub struct Matrix<T: Clone> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Clone> Matrix<T> {
    /// Initializes the matrix with a given value
    pub fn with_val(rows: usize, cols: usize, val: T) -> Self {
        Matrix {
            data: vec![val; rows * cols],
            rows,
            cols,
        }
    }

    /// Returns the number of rows in the matrix
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the matrix
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the total number of elements in the matrix
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Retrieves a reference to the element of the matrix given a coordinate tuple, where the
    /// first element is the row index and the second element is the column index.  The function
    /// returns a Option<&T> where it is None if the provided coordinates are out of bounds.
    pub fn get(&self, coord: (usize, usize)) -> Option<&T> {
        self.data.get((self.cols * coord.0) + coord.1)
    }

    /// Similar to `get`, except the returned reference is mutable.
    pub fn get_mut(&mut self, coord: (usize, usize)) -> Option<&mut T> {
        self.data.get_mut((self.cols * coord.0) + coord.1)
    }

    /// Sets the value at the provided coordinate to the new value.
    pub fn set(&mut self, coord: (usize, usize), new_val: T) -> Result<(), &str> {
        let x = match self.get_mut(coord) {
            Some(x) => x,
            None => {
                return Err("Invalid Index");
            }
        };
        *x = new_val;
        Ok(())
    }
}

impl<T: Clone + Default> Matrix<T> {
    /// Creates a new matrix filled with the default value from T
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![T::default(); rows * cols],
            rows,
            cols,
        }
    }
}

/// Implement the index trait for Matrix.  The first element in the tuple is the row indx and the
/// second element is the column index.
impl<T: Clone> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, coord: (usize, usize)) -> &Self::Output {
        &self.data[(self.cols * coord.0) + coord.1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mat = Matrix::<i32>::new(10, 8);

        assert_eq!(mat.cols(), 8);
        assert_eq!(mat.rows(), 10);
        assert_eq!(mat.size(), 80);
        assert_eq!(mat[(0, 0)], 0);
    }

    #[test]
    fn test_new_size_0() {
        let mat = Matrix::<f64>::new(0, 1);
        assert_eq!(mat.size(), 0);

        let mat = Matrix::<usize>::new(100, 0);
        assert_eq!(mat.size(), 0);
    }

    #[test]
    fn test_with_val() {
        let mat = Matrix::<&str>::with_val(2, 3, "A");

        assert_eq!(mat.cols(), 3);
        assert_eq!(mat.rows(), 2);
        assert_eq!(mat.size(), 6);
        assert_eq!(mat[(0, 2)], "A");
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let mat = Matrix::<f32>::new(100, 100);
        mat[(100, 0)];
    }

    #[test]
    fn test_get() {
        let mat = Matrix::<i32>::new(10, 10);

        assert_eq!(mat.get((0, 0)), Some(&0));
        assert_eq!(mat.get((100, 200)), None);
    }

    #[test]
    fn test_get_mut() {
        let mut mat = Matrix::<i32>::new(10, 10);
        let x = mat.get_mut((0, 0)).unwrap();
        *x = 1;

        assert_eq!(mat.get((0, 0)), Some(&1));
        assert_eq!(mat.get_mut((100, 200)), None);
    }

    #[test]
    fn test_set() {
        let mut mat = Matrix::<i32>::new(10, 10);
        assert_eq!(mat.set((0, 0), 10), Ok(()));
        assert_eq!(mat.get((0, 0)), Some(&10));
        assert_eq!(mat.set((100, 100), 20), Err("Invalid Index"));
    }
}
