// use rand::Rng;
use std::collections::HashMap;
use std::fmt;

pub type Coords = (u8, u8);

#[derive(Debug)]
pub struct Cell {
    pub number: u8,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            number: 1,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub size: Coords,
    pub game_ended: bool,
    pub has_won: bool,
    selected_pair: (Option<Coords>, Option<Coords>),
    cells: HashMap<Coords, Cell>,
}

impl Board {
    pub fn new(size: Coords) -> Self {
        let mut b = Board {
            size,
            game_ended: false,
            has_won: false,
            selected_pair: (None, None),
            cells: HashMap::new(),
        };

        for y in 0..size.1 {
            for x in 0..size.0 {
                let pos = (x, y);
                let mut cell = Cell::new();
                cell.number = 1;
                b.cells.insert(pos, cell);
            }
        }

        b
    }
    
    pub fn get_cell(self: &Self, pos: &Coords) -> Option<&Cell> {
        self.cells.get(pos)
    }
    
    pub fn get_cell_mut(self: &mut Self, pos: &Coords) -> Option<&mut Cell> {
        self.cells.get_mut(pos)
    }
    
    pub fn has_won(self: &Self) -> bool {
        false
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut st = String::new();
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let pos = (x, y);
                let cell = self.get_cell(&pos).unwrap();
                st.push(char::from(cell.number + 48));
            }
            st.push('\n');
        }
        write!(f, "{}", st)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_initial_board_display() {
        let b = Board::new((3, 3));
        
        assert_eq!(format!("{}", b), "111\n111\n111\n");
    }
}
