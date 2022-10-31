use std::ops;



pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    cells: Vec<Vec<T>>,
}

impl<T: Copy + std::default::Default> Grid<T>{
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![T::default(); width]; height];
        Grid { width, height, cells }
    }

    pub fn from_vec(cells: Vec<Vec<T>>) -> Self {
        let width = cells[0].len();
        let height = cells.len();
        Grid { width, height, cells }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.height && col < self.width {
            Some(&self.cells[row][col])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.height && col < self.width {
            Some(&mut self.cells[row][col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> Option<T> {
        if row < self.height && col < self.width {
            let old = std::mem::replace(&mut self.cells[row][col], value);
            Some(old)
        } else {
            None
        }
    }

    pub fn fill(&mut self, value: T) {
        for row in 0..self.height {
            for col in 0..self.width {
                self.cells[row][col] = value.clone();
            }
        }
    }

    pub fn to_vec(&self) -> Vec<Vec<T>> {
        self.cells.clone()
    }
}

impl ops::Index<usize> for Grid<i32> {
    type Output = Vec<i32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl ops::IndexMut<usize> for Grid<i32> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}