mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
struct GameState {
    cells: Vec<u32>,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl GameState {
    pub fn new() -> GameState {
        let width = 4;
        let height = 4;
        let cells = (0..width * height)
            .map(|i| if i % 3 == 0 { 2 } else { 0 })
            .collect();
        GameState {
            cells,
            width,
            height,
        }
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn up(&mut self, first: bool) {
        let mut new_state = self.cells.clone();
        for row in (1..self.height).rev() {
            for col in 0..self.width {
                let last_row_index = ((row - 1) * (self.height) + col) as usize;
                let current_row_index = (row * (self.height) + col) as usize;
                if new_state[last_row_index] == 0 {
                    new_state[last_row_index] = new_state[current_row_index];
                    new_state[current_row_index] = 0;
                }
                if new_state[current_row_index] == new_state[last_row_index] {
                    new_state[last_row_index] *= 2;
                    new_state[current_row_index] = 0;
                }
                if new_state[current_row_index] == 0 && row == self.height - 1 && first {
                    new_state[current_row_index] = 2;
                }
            }
        }
        if self.cells != new_state {
            self.cells = new_state;
            self.up(false);
        }
    }

    pub fn down(&mut self, first: bool) {
        let mut new_state = self.cells.clone();
        for row in (0..(self.height - 1)) {
            for col in 0..self.width {
                let last_row_index = ((row + 1) * (self.height) + col) as usize;
                let current_row_index = (row * (self.height) + col) as usize;
                if new_state[last_row_index] == 0 {
                    new_state[last_row_index] = new_state[current_row_index];
                    new_state[current_row_index] = 0;
                }
                if new_state[current_row_index] == new_state[last_row_index] {
                    new_state[last_row_index] *= 2;
                    new_state[current_row_index] = 0;
                }
                if new_state[current_row_index] == 0 && row == 0 && first {
                    new_state[current_row_index] = 2;
                }
            }
        }
        if self.cells != new_state {
            self.cells = new_state;
            self.down(false);
        }
    }

    pub fn left(&mut self, first: bool) {
        let mut new_state = self.cells.clone();
        for col in (1..self.width).rev() {
            for row in 0..self.height {
                let last_col_index = (row * self.height + col - 1) as usize;
                let current_col_index = (row * self.height + col) as usize;
                if new_state[last_col_index] == 0 {
                    new_state[last_col_index] = new_state[current_col_index];
                    new_state[current_col_index] = 0;
                }
                if new_state[current_col_index] == new_state[last_col_index] {
                    new_state[last_col_index] *= 2;
                    new_state[current_col_index] = 0;
                }
                if new_state[current_col_index] == 0 && col == self.width - 1 && first {
                    new_state[current_col_index] = 2;
                }
            }
        }
        if self.cells != new_state {
            self.cells = new_state;
            self.left(false);
        }
    }

    pub fn right(&mut self, first: bool) {
        let mut new_state = self.cells.clone();
        for col in 0..(self.width - 1) {
            for row in 0..self.height {
                let last_col_index = (row * self.height + col + 1) as usize;
                let current_col_index = (row * self.height + col) as usize;
                if new_state[last_col_index] == 0 {
                    new_state[last_col_index] = new_state[current_col_index];
                    new_state[current_col_index] = 0;
                }
                if new_state[current_col_index] == new_state[last_col_index] {
                    new_state[last_col_index] *= 2;
                    new_state[current_col_index] = 0;
                }
                if new_state[current_col_index] == 0 && col == 0 && first {
                    new_state[current_col_index] = 2;
                }
            }
        }
        if self.cells != new_state {
            self.cells = new_state;
            self.right(false);
        }
    }

    pub fn check_game_over(&mut self) -> bool {
        let mut flag1 = false;
        for col in 0..(self.width - 1) {
            for row in 0..self.height {
                let last_col_index = (row * self.height + col + 1) as usize;
                let current_col_index = (row * self.height + col) as usize;
                if self.cells[last_col_index] == self.cells[current_col_index] {
                    flag1 = true;
                    break;
                }
            }
        }

        for row in 0..(self.height - 1) {
            for col in 0..self.width {
                let last_row_index = ((row + 1) * (self.height) + col) as usize;
                let current_row_index = (row * (self.height) + col) as usize;
                if self.cells[last_row_index] == self.cells[current_row_index] {
                    flag1 = true;
                    break;
                }
            }
        }
        if self.cells.contains(&0) || flag1 {
            return false;
        } else {
            return true;
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 {
                    format!("{:>8}", "[]")
                } else {
                    format!("{:>8}", cell)
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
