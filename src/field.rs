mod place;

use place::*;
use itertools::Itertools;
use rand::Rng;

use super::{N, SCALE, BOMBS_NUMBER};

pub struct Field { 
    pub field: [[Place; N]; N],
}

impl Field {
    pub fn new()->Self {
        let place = Place::new();
        let mut fld = Self { field: [[place; N]; N] };
        fld.place_bombs();
        fld
    }

    pub fn uncover(& mut self, x: i16, y: i16) {
        if self.is_in_range(x, y) {
            let place = & mut self.field[x as usize][y as usize];
            if place.is_hidden() {
                place.uncover();
                if place.is_empty() {
                    for (dx,dy) in [-1, 0, 1].iter().cartesian_product([-1, 0 , 1].iter()) {
                        self.uncover(x + dx, y + dy);
                    }
                } 
            }
        }
    }

    fn place_bombs(& mut self) {
        for _i in 0..BOMBS_NUMBER {
            let x:usize = rand::thread_rng().gen_range(0..N);
            let y:usize = rand::thread_rng().gen_range(0..N);
            
            if !self.field[x][y].is_bomb() {
                self.build_numbers(x, y); 
                self.field[x][y].set_bomb();                       
            }
        }
    }

    fn is_in_range(&self, x: i16, y : i16) -> bool{
        x >= 0 && x < (N as i16) && y >= 0 && y < (N as i16)
    }

    fn build_numbers(& mut self, x: usize, y: usize) {
        for (dx,dy) in [-1, 0, 1].iter().cartesian_product([-1, 0 , 1].iter()) {
            let x_new = (x as i16) + dx;
            let y_new = (y as i16) + dy;
            if self.is_in_range(x_new, y_new) && !self.field[x_new as usize][y_new as usize].is_bomb() {
                self.field[x_new as usize][y_new as usize].number += 1;
            }
        }
    }

    pub fn is_solved(&self) -> bool {
        let mut solved = true;
        for x in 0..N {
            for y in 0..N {
                if self.field[x][y].is_hidden() && !self.field[x][y].is_bomb() {
                    solved = false;
                    break;
                }
            }
        }
        solved
    }

    pub fn is_bomb(&self, x: usize, y: usize)->bool {
        self.field[x][y].is_bomb()
    }

    pub fn toggle_marker(& mut self, x: usize, y: usize) {
        self.field[x][y].toggle_marker();
    }
}
