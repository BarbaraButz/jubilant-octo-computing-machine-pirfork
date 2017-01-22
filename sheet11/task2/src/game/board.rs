use std::fmt;

pub struct Board(pub [[Field; 3]; 3]);

impl Board {
    pub fn new() -> Self {
        Board([[Field::Empty; 3]; 3])
    }

    pub fn empty_indices(&self) -> Vec<(usize, usize)> {
        let mut indices = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                indices.push((i, j));
            }
        }
        indices.into_iter().filter(|&(i, j)| self.0[i][j] == Field::Empty).collect()
    }

    pub fn full(&self) -> bool {
        self.empty_indices().len() == 0
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Symbol {
    O,
    X,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Field {
    Empty,
    Occupied(Symbol),
}