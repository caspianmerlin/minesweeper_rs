use std::fmt::{Display, Formatter};

use crate::{config::Config, util::RandomNumberGenerator};

//use crate::{win32::get_random, pref::Preferences, utils::Measurements, graphics::{RESIZE, DISPLAY}};

const MAXFIELDSIZE: usize = 27 * 32;
const VISITED: u8 = 0b00000001;
const MINE: u8 = 0b00000010;
const FLAGGED: u8 = 0b00000100;
const Q_MARKED: u8 = 0b00001000;
const EXPLODED: u8 = MINE | VISITED;
const DO_NOT_UNCOVER: u8 = FLAGGED | MINE | VISITED;
pub const ADJUST: i32 = 1;
pub const RESIZE: i32 = 2;
pub const DISPLAY: i32 = 4;

pub struct GameBoard {
    pub grid: [u8; MAXFIELDSIZE],
    pub num_rows: usize,
    pub num_columns: usize,
    pub num_mines: usize,
    pub num_uncovered_squares: usize,
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for r in 0..self.num_rows {
            for c in 0..self.num_columns {
                let index = (r *  self.num_columns) + c;
                let mut symbol: char = if self.grid[index] & MINE == MINE { 'M' }
                else {
                    let adj = (self.grid[index] >> 4);
                    char::from_u32(adj as u32 + 48).unwrap()
                };
                if symbol == '0' && self.grid[index] & VISITED == VISITED { symbol = 'X'; }
                write!(f, "{}", symbol);
            }
            write!(f, "\r\n");
        }
        write!(f, "\r\n")
    }
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            grid: [0; MAXFIELDSIZE],
            num_rows: 0,
            num_columns: 0,
            num_mines: 0,
            num_uncovered_squares: 0,
        }
    }
    pub fn clear(&mut self) {
        for square in self.grid.iter_mut() {
            *square = 0;
        }
    }
    pub fn setup(&mut self, config: &Config, random_number_generator: &mut dyn RandomNumberGenerator) -> i32 {

        self.clear();
        let (config_width, config_height) = config.difficulty.dimensions();
        let adjust = if config_width != self.num_columns as u32 || config_height != self.num_rows as u32 {
            RESIZE | DISPLAY
        } else {
            DISPLAY
        };

        self.num_rows = config_height as usize;
        self.num_columns = config_width as usize;
        self.num_mines = config.difficulty.num_mines() as usize;
        self.num_uncovered_squares = self.num_rows * self.num_columns - self.num_mines;

        for _ in 0..self.num_mines {
            let (rand_row, rand_column) = loop {
                let rand_row = random_number_generator.random_u32(config_height);
                let rand_column = random_number_generator.random_u32(config_width);
                if (!self.is_mine(rand_row, rand_column)) {
                    break (rand_row, rand_column);
                }
            };
            self.set_mine(rand_row, rand_column);   
        }
        adjust
    }

    pub fn get_display(&mut self, row: usize, column: usize) -> usize {
        if self.is_mine(row, column) {
            10
        } else{
            let adj = self.get_adjacent(row, column);
            if adj == 0 {
                15
            } else {
                adj as usize
            }
        } 
    }

    pub fn get_square(&mut self, row: usize, column: usize) -> &mut u8 {
        &mut self.grid[(row * self.num_columns) + column]
    }

    pub fn is_visited(&mut self, row: usize, column: usize) -> bool {
        *self.get_square(row, column) & VISITED == VISITED
    }
    pub fn is_mine(&mut self, row: usize, column: usize) -> bool {
        *self.get_square(row, column) & MINE == MINE
    }
    pub fn is_flagged(&mut self, row: usize, column: usize) -> bool {
        *self.get_square(row, column) & FLAGGED == FLAGGED
    }
    pub fn is_q_marked(&mut self, row: usize, column: usize) -> bool {
        *self.get_square(row, column) & Q_MARKED == Q_MARKED
    }

    pub fn set_visited(&mut self, row: usize, column: usize) {
        *self.get_square(row, column) |= VISITED;
    }
    pub fn set_mine(&mut self, row: usize, column: usize) {
        *self.get_square(row, column) |= MINE;
    }
    pub fn set_flagged(&mut self, row: usize, column: usize) {
        *self.get_square(row, column) |= FLAGGED;
    }
    pub fn set_q_marked(&mut self, row: usize, column: usize) {
        *self.get_square(row, column) |= Q_MARKED;
    }

    pub fn get_adjacent(&mut self, row: usize, column: usize) -> u8 {
        (*self.get_square(row, column) >> 4)
    }
    pub fn set_adjacent(&mut self, row: usize, column: usize, value: u8) {
        let value = value << 4;
        *self.get_square(row, column) &= 0b00001111;
        *self.get_square(row, column) |= value;
    }
    pub fn mask_matches_exact(&mut self, row: usize, column: usize, mask: u8) -> bool {
        (*self.get_square(row, column) & mask) == mask
    }
    pub fn mask_matches_any(&mut self, row: usize, column: usize, mask: u8) -> bool {
        (*self.get_square(row, column) & mask) > 0
    }

    pub fn adjacent_square_indices(&mut self, row: usize, column: usize) -> Vec<(usize, usize)>{
        let mut vec = Vec::with_capacity(8);
        let (up, down, left, right) = (row > 0, row < self.num_rows - 1, column > 0, column < self.num_columns - 1);
        if up {
            vec.push((row -1, column));
            if left {
                vec.push((row - 1, column - 1));
            }
            if right {
                vec.push((row - 1, column + 1));
            }
        }
        if left {
            vec.push((row, column - 1));
        }
        if right {
            vec.push((row, column + 1));
        }
        if down  {
            vec.push((row +1, column));
            if left {
                vec.push((row + 1, column - 1));
            }
            if right {
                vec.push((row + 1, column + 1));
            } 
        }
        vec
    }

    pub fn adjacent_square_indices_no_diag(&mut self, row: usize, column: usize) -> Vec<(usize, usize)>{
        let mut vec = Vec::with_capacity(8);
        let (up, down, left, right) = (row > 0, row < self.num_rows - 1, column > 0, column < self.num_columns - 1);
        if up {
            vec.push((row -1, column));
        }
        if left {
            vec.push((row, column - 1));
        }
        if right {
            vec.push((row, column + 1));
        }
        if down  {
            vec.push((row +1, column));
        }
        vec
    }

    pub fn left_click(&mut self, row: usize, column: usize) -> bool {
        if self.is_visited(row, column) || self.is_flagged(row, column){
            return false;
        } else {
            if self.is_mine(row, column) {
                self.set_visited(row, column);
                return true;
            }

            //If it's not been visited, isn't a flag and isn't a mine, it is an empty square that should be revealed.
            else {
                //We need to keep a count of how many squares have been uncovered this turn.
                let mut squares_uncovered_this_turn: usize = 0;

                //If this square is a numbered square (i.e. it is adjacent to at least one mine), we uncover it but no other squares.
                if self.get_adjacent(row, column) > 0 {
                    self.set_visited(row, column);
                    squares_uncovered_this_turn += 1;
                }

                //Otherwise, the square is 'blank'. If that is the case, we call the recursive method on it.
                //We pass in our count, this will be recursively increased as further squares are uncovered.
                else {
                    self.set_visited(row, column);
                    squares_uncovered_this_turn += 1;
                    self.uncover_adjacent_empty_squares(row, column, &mut squares_uncovered_this_turn);
                }

                //Finally, subtract the number of squares uncovered this turn from the total number of squares left to uncover.
                self.num_uncovered_squares -= squares_uncovered_this_turn;
                println!("{} squares uncovered this turn. {} squares remaining.", squares_uncovered_this_turn, self.num_uncovered_squares);
            }
        }
        false
    }

    pub fn uncover_adjacent_empty_squares(&mut self, row: usize, column: usize, num_uncovered: &mut usize) {
        for (adj_row, adj_col) in self.adjacent_square_indices(row, column) {
            // If the adjacent square has either been visited, has a mine or a flag, do nothing.
            if self.mask_matches_any(adj_row, adj_col, DO_NOT_UNCOVER) {
                continue;
            }
            //Otherwise, we'll mark the square as visited in any case.
            self.set_visited(adj_row, adj_col);
            *num_uncovered += 1;

            //If and only if the adjacent square is a blank one, we'll call this recursive method on it as well.
            if self.get_adjacent(adj_row, adj_col) == 0 {
                self.uncover_adjacent_empty_squares(adj_row, adj_col, num_uncovered);
            }
        }
    }

    pub fn calculate_adjacent_mines(&mut self) {
        for r in 0..self.num_rows {
            for c in 0..self.num_columns {
                let mut mines_found = 0;
                let adjacent_squares = self.adjacent_square_indices(r, c);
                for (adj_r, adj_c) in adjacent_squares {
                    if self.is_mine(adj_r, adj_c) && !self.is_mine(r, c) {
                        mines_found += 1;
                    }
                }
                self.set_adjacent(r, c, mines_found);
            }
        }
    } 
}

