use rand::Rng;
use std::fmt;
use std::fmt::Display;
use std::{thread, time};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
//had to use PartialEq for the fmt function
enum Cell {
    Alive = 1,
    Dead = 0,
}

struct Board {
    height: u32,
    width: u32,
    cells: Vec<Cell>,
}

impl Board{
    fn get_index(&self, row: u32, col: u32) -> usize {
        // math
        (row * self.width + col) as  usize
    }

    fn count_living_neighbors(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        // takes delta values and uses modulus to enable "wrapping" around the board.
        // If the row or col is 0, it'll add the (height/width - 1), putting the cell
        // on the opposing side, and therefore "wrapping it"
        // tinyurl.com/deltaexample
        for delta_row in [self.height-1, 0, 1].iter().cloned() {
            for delta_col in [self.width -1, 0, 1].iter().cloned() {
                //skipping the original cell in the sum
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (delta_row + row) % self.height;
                let neighbor_col = (delta_col + col) % self.width;
                // add sum of neighboring cells alive/dead status
                count += self.cells[self.get_index(neighbor_row,neighbor_col)] as u8;
            }
        }
        count
    }

    fn generation(&mut self) {
        // makes a copy of cells vector 
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let cell = self.cells[self.get_index(row,col)];
                let living_neighbors = self.count_living_neighbors(row,col);

                // finds the current cell's next state
                let next_cell = match (cell,living_neighbors) {
                    // if a cell has less than 2 alive neighbors, it DIES
                    (Cell::Alive, 0) | (Cell::Alive, 1) => Cell::Dead,
                    // if a cell has 2 or 3 living neighbors, it LIVES
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // if a cell has more than 3 living neighbors, overpopulation and DIES
                    (Cell::Alive, _)  => Cell::Dead,
                    // if a dead cell has 3 living neighbors, reproduction and LIVES
                    (Cell::Dead, 3) => Cell::Alive,
                    // otherwise, still dead
                    (Cell::Dead, _) => Cell::Dead,
                };

                // updates the cell's state in the upcoming generation
                next[self.get_index(row,col)] = next_cell;

            }
        }
        // new generation is set as the active generation
        self.cells = next;
    }

    fn output(&self) -> String {
        self.to_string()
    }

    fn create_board(h: u32, w: u32) -> Board {
        let height = h;
        let width = w;
        let cells = (0..width*height).map(|_| {
                                                        // randomizes the cells with a 20% of birth
                                                        let mut rng = rand::thread_rng();
                                                        if rng.gen_range(0, 11) > 8 {
                                                            Cell::Alive
                                                        }
                                                        else {
                                                            Cell::Dead
                                                        }
                                                        }).collect();
        Board {
            height,
            width,
            cells,
        }
    }
}

// found in online "Game of life in Rust" tutorial b/c i didn't know how to
// print the cells vector of enums
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut test = Board::create_board(20,100);
    let mut x = 0;
    loop {
        x += 1;
        test.generation();
        println!("Generation {}", x);
        println!("{}", test.output());
        thread::sleep(time::Duration::from_millis(250));
    }
}