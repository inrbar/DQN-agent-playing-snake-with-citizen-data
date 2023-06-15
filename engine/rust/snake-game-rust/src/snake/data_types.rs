use std::{ops::{Index, IndexMut, Add, AddAssign}, fmt::Display};

/// generic wrapper for a matrix type with implementations
/// for printing and python style indexing
/// 
/// you can read a write to the specific matrix elements
/// ```
/// use snake_game_rust::snake::data_types::Matrix;
/// let mut a = Matrix::zeros((10,10));
/// 
/// a[(3,4)] = 5;
/// 
/// ```
#[derive(Clone)]
pub struct Matrix{
    pub matrix : Vec<Vec<i32>>,
    pub size : (usize,usize)
}

impl Matrix {
    
    /// generates a matrix of zeroes with a given size
    /// 
    /// # Panics
    /// 
    /// should only panic if the supplied size is of wrong type
    /// 
    /// # Examples
    /// 
    /// ```
    /// use snake_game_rust::snake::data_types::Matrix;
    /// let a = Matrix::zeros((10,10));
    /// 
    /// ```
    pub fn zeros(size : (usize,usize)) -> Matrix{

        Matrix {
            matrix : vec![vec![0;size.0];size.1],
            size : size
        }

    }

}

impl Display for Matrix{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.matrix {
            for col in row {
                write!(f, "{} ", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<(usize,usize)> for Matrix {
    type Output = i32;
    fn index(&self, index: (usize,usize)) -> &Self::Output {
        &self.matrix[index.0][index.1]
    }
}

impl Index<(i32,i32)> for Matrix {
    type Output = i32;
    fn index(&self, index: (i32,i32)) -> &Self::Output {
        let i = match usize::try_from(index.0) {
            Ok(x) => x,
            Err(_) => match usize::try_from(index.0%(i32::try_from(self.size.0)).unwrap()) {
                Ok(x) => x,
                Err(_) => panic!("failed to cast int ") 
            }  
        };
        let j = match usize::try_from(index.1) {
            Ok(x) => x,
            Err(_) => match usize::try_from(index.1%(i32::try_from(self.size.1)).unwrap()) {
                Ok(x) => x,
                Err(_) => panic!("failed to cast int ") 
            }  
        };
        &self[(i,j)]
    }
}

impl IndexMut<(i32,i32)> for Matrix {
    fn index_mut(&mut self, index: (i32,i32)) -> &mut Self::Output {
        let i = match usize::try_from(index.0) {
            Ok(x) => x,
            Err(_) => match usize::try_from(index.0%(i32::try_from(self.size.0)).unwrap()) {
                Ok(x) => x,
                Err(_) => panic!("failed to cast int ") 
            }  
        };
        let j = match usize::try_from(index.1) {
            Ok(x) => x,
            Err(_) => match usize::try_from(index.1%(i32::try_from(self.size.1)).unwrap()) {
                Ok(x) => x,
                Err(_) => panic!("failed to cast int ") 
            }  
        };
        &mut self[(i,j)]
    }
}

impl IndexMut<(usize,usize)> for Matrix{
    fn index_mut(&mut self, index: (usize,usize)) -> &mut Self::Output {
        &mut self.matrix[index.0][index.1]
    }
}

/// generic 2D vector struct with implemenatations
/// for equality and addition and a method for multiplying by a number
#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct Vec2{
    pub x : i32,
    pub y : i32
}

impl Add for Vec2{
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2{x : self.x + rhs.x,y : self.y + rhs.y}
    }
}

impl AddAssign for Vec2{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}


impl Vec2{


    /// a simple method to multiply both components
    /// by a number
    /// 
    /// # Examples
    /// ```
    /// use snake_game_rust::snake::data_types::Vec2;
    /// 
    /// let a = Vec2 { x : 1, y : 2};
    /// 
    /// let b = a.mul_by_i32(2);
    /// 
    /// assert!(b ==  Vec2{ x : 2, y : 4});
    /// 
    /// ```
    pub fn mul_by_i32(self, n : i32) -> Vec2{
        Vec2 { x: self.x * n, y: self.y * n }
    }

}
