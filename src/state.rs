use comfy::play_sound;
use rand::prelude::*;
use std::collections::HashMap;
use std::fmt;

pub type Coords = (u8, u8);

#[derive(Debug)]
pub struct LevelParams {
    pub size: Coords,
    pub penalty_countdown: f64,
    pub fill_countdown: f64,
    pub goal_number: u8,
}

pub const NUM_LEVELS: u8 = 4;

fn setup_new_level(level: u8) -> LevelParams {
    match level {
        //1 => LevelParams { size: (4, 4), penalty_countdown: 75.0, fill_countdown: 2.0, goal_number: 3 },
        1 => LevelParams { size: (5, 5), penalty_countdown: 75.0, fill_countdown: 2.0, goal_number: 5 },
        2 => LevelParams { size: (7, 7), penalty_countdown: 60.0, fill_countdown: 2.0, goal_number: 6 },
        3 => LevelParams { size: (8, 8), penalty_countdown: 45.0, fill_countdown: 2.0, goal_number: 7 },
        _ => LevelParams { size: (9, 9), penalty_countdown: 30.0, fill_countdown: 1.5, goal_number: 8 },
    }
}

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
pub struct State {
    pub game_paused: bool,
    pub selection: Option<Coords>,
    pub current_level: u8,
    pub matches: u16,
    pub mistakes: u16,
    pub elapsed_time: f64,
    pub penalty_countdown: f64,
    pub fill_countdown: f64,
    pub level_params: LevelParams,
    cells: HashMap<Coords, Cell>,
    rng: ThreadRng,
}

impl State {
    pub fn new_() -> Self {
        let level_params = setup_new_level(1);
        State {
            game_paused: true,
            selection: None,
            current_level: 0,
            matches: 0,
            mistakes: 0,
            elapsed_time: 0.0,
            penalty_countdown: 0.0,
            fill_countdown: 0.0,
            level_params,
            cells: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }
    
    fn populate_board(self: &mut Self) {
        self.penalty_countdown = self.level_params.penalty_countdown;
        self.fill_countdown = self.level_params.fill_countdown;
        self.cells.clear();
        for y in 0..self.level_params.size.1 {
            for x in 0..self.level_params.size.0 {
                let pos = (x, y);
                let mut cell = Cell::new();
                cell.number = 1;
                self.cells.insert(pos, cell);
            }
        }
    }
    
    pub fn at_last_level(self: &Self) -> bool {
        self.current_level == NUM_LEVELS
    } 
    
    pub fn increase_level(self: &mut Self) -> bool {
        if self.at_last_level() {
            return false;
        }
        self.current_level += 1;
        self.level_params = setup_new_level(self.current_level);
        self.populate_board();
        true
    }
    
    pub fn add_to_selection(self: &mut Self, pos: &Coords) -> bool {
        match self.selection {
            None => {
                self.selection = Some(pos.clone());
                return false;
            },
            Some(prev_pos) => {
                if prev_pos == *pos {
                    //println!("UNSELECTED");
                    self.selection = None;
                    return false;
                }
                
                let first_cells_number = self.get_cell(&prev_pos).unwrap().number;
                let second_cell_number = self.get_cell(pos).unwrap().number;
                
                if first_cells_number == second_cell_number && first_cells_number != 0 {
                    self.get_cell_mut(&prev_pos).unwrap().number = 0;
                    self.get_cell_mut(pos).unwrap().number += 1;
                    self.matches += 1;
                    //println!("MATCHED {}s MATCHES: {}", first_cells_number, self.matches);
                    play_sound("merge");
                    self.selection = None;
                    return true;
                } else {
                    self.mistakes += 1;
                    self.penalty_countdown -= 10.0;
                    play_sound("mistake");
                    //println!("MISTAKES: {}", self.mistakes);
                    self.selection = None;
                    return false;
                }
            }
        }
    }
    
    pub fn get_cell(self: &Self, pos: &Coords) -> Option<&Cell> {
        self.cells.get(pos)
    }
    
    pub fn get_cell_mut(self: &mut Self, pos: &Coords) -> Option<&mut Cell> {
        self.cells.get_mut(pos)
    }
    
    pub fn handle_countdowns(self: &mut Self, delta: f64) {
        if self.game_paused {
            return;
        }
        
        let p_sec_ = self.penalty_countdown.round();
        
        self.elapsed_time += delta;
        self.penalty_countdown -= delta;
        self.fill_countdown -= delta;
        
        if self.penalty_countdown <= 0.0 {
            self.penalty_countdown = self.level_params.penalty_countdown;
            self.clean_cells(0.25);
            play_sound("penalty");
        }
        
        let p_sec = self.penalty_countdown.round();
        if p_sec != p_sec_ && self.penalty_countdown > 0.5 && self.penalty_countdown < 5.5 {
            play_sound("incoming_tick");
        }
        
        if self.fill_countdown <= 0.0 {
            self.fill_countdown = self.level_params.fill_countdown;
            if self.fill_empty_cell() {
                play_sound("fill");
            }
        }
    }
    
    fn get_matching_positions<F>(self: &Self, closure: F) -> Vec<Coords>
    where F: Fn(&Cell) -> bool {
        let mut result = Vec::new();
        for y in 0..self.level_params.size.1 {
            for x in 0..self.level_params.size.0 {
                let pos = (x, y);
                let cell = self.get_cell(&pos).unwrap();
                if closure(cell) {
                    result.push(pos);
                }
            }
        }
        result
    }
    
    pub fn clean_cells(self: &mut Self, ratio:f64) {
        let filled_positions = self.get_matching_positions(|c| c.number > 0);
        for pos in filled_positions {
            let r = self.rng.gen_range(0.0..1.0);
            if r < ratio {
                //println!("CLEARING {:?}", pos);
                self.get_cell_mut(&pos).unwrap().number = 0;
            }
        }
    }
    
    pub fn fill_empty_cell(self: &mut Self) -> bool {
        let empty_positions = self.get_matching_positions(|c| c.number == 0);
        if empty_positions.len() == 0 {
            return false;
        }
        let i = self.rng.gen_range(0..empty_positions.len());
        let pos = empty_positions[i];
        //println!("FILLING {:?} WITH 1", pos);
        self.get_cell_mut(&pos).unwrap().number = 1;
        true
    }
    
    pub fn has_won(self: &Self) -> bool {
        for y in 0..self.level_params.size.1 {
            for x in 0..self.level_params.size.0 {
                let pos = (x, y);
                let cell = self.get_cell(&pos).unwrap();
                if cell.number == self.level_params.goal_number {
                    return true;
                }
            }
        }
        false
    }
    
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut st = String::new();
        for y in 0..self.level_params.size.1 {
            for x in 0..self.level_params.size.0 {
                let pos = (x, y);
                let cell = self.get_cell(&pos).unwrap();
                st.push(num_to_char(cell.number));
            }
            st.push('\n');
        }
        write!(f, "{}", st)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn check_initial_board_display() {
//         let b = Board::new((3, 3));
        
//         assert_eq!(format!("{}", b), "111\n111\n111\n");
//     }
    
//     #[test]
//     fn valid_match() {
//         let mut b = Board::new((3, 3));
        
//         b.add_to_selection(&(0, 0));
//         b.add_to_selection(&(1, 0));
        
//         assert_eq!(format!("{}", b), "021\n111\n111\n");
//     }
// }
