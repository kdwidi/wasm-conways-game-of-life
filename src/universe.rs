use js_sys::Array;
use wasm_bindgen::prelude::wasm_bindgen;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;

use crate::utils::set_panic_hook;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if col == 0 {
            self.width - 1
        } else {
            col - 1
        };

        let east = if col == self.width - 1 {
            0
        } else {
            col + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, col);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, col);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    /*
       pub fn get_cells(&self) -> FixedBitSet {
           self.cells.clone()
       }

       pub fn set_cell_alive(&mut self, cells: &Vec<usize>) {
           for i in cells.iter() {
               self.cells.set(*i, true);
           }
       }
    */
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        let bit_size = (width * height) as usize;
        if height == 0 || width == 0 {
            set_panic_hook("error: value of width or height is `0`");
        }

        Self {
            width,
            height,
            cells: FixedBitSet::with_capacity(bit_size),
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(row, col);
                next.set(
                    index,
                    match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 2 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }
        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn set_cells(&mut self, cells: Array) {
        for i in cells.iter() {
            self.cells.set(i.as_f64().unwrap() as usize, true);
        }
    }

    pub fn clear_cells(&mut self) {
        self.cells.clear();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let index = self.get_index(row, column);
        self.cells.toggle(index);
    }
}
