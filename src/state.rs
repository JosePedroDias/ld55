// use rand::Rng;
use std::collections::HashMap;
use std::fmt;

pub type Coords = (u8, u8);

pub fn num_to_char(number: u8) -> char {
    char::from(number + 48)
}

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
    selection: Vec<Coords>,
    cells: HashMap<Coords, Cell>,
}

impl Board {
    pub fn new(size: Coords) -> Self {
        let mut b = Board {
            size,
            game_ended: false,
            has_won: false,
            selection: vec!(),
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
    
    pub fn add_to_selection(self: &mut Self, pos: &Coords) -> bool {
        if self.selection.len() == 0 {
            self.selection.push(pos.clone());
            return false;
        }
        
        let first_pos = self.selection[0];
        if first_pos == *pos {
            self.selection.remove(0);
            return false;
        } else {
            // is this a merge?
            
            let first_cells_number = self.get_cell(&first_pos).unwrap().number;
            let second_cell_number = self.get_cell(pos).unwrap().number;
            
            if first_cells_number == second_cell_number {
                println!("MATCHING {}s", first_cells_number);
                self.get_cell_mut(&first_pos).unwrap().number = 0;
                self.get_cell_mut(pos).unwrap().number += 1;
                self.selection.remove(0);
                return true;
            } else {
                println!("NO MATCH");
                self.selection.remove(0);
                return false;
            }
        }
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
                st.push(num_to_char(cell.number));
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
    
    #[test]
    fn valid_match() {
        let mut b = Board::new((3, 3));
        
        b.add_to_selection(&(0, 0));
        b.add_to_selection(&(1, 0));
        
        assert_eq!(format!("{}", b), "021\n111\n111\n");
    }
}
