pub struct Board([[Field; 3]; 3]);

impl Board {
    fn new() -> Self {
        Board([[Field::Empty; 3]; 3])
    }

    fn empty_indices(&self) -> Vec<(usize, usize)> {
        unimplemented!();
    }

    fn full(&self) -> bool {
        self.empty_indices().len() == 0
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Symbol {
    O,
    X,
}

#[derive(Debug, Copy, Clone)]
pub enum Field {
    Empty,
    Occupied(Symbol),
}
