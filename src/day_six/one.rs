use std::collections::HashSet;

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;

const AVAILABLE_SPOT: char = '.';
const OBSTRUCTION: char = '#';
const OUTSIDE: char = '0';

pub struct Guard {
    x: usize,
    y: usize,
    orientation: usize,
    visited_positions: HashSet<(usize, usize)>,
    is_outside: bool,
}

impl Guard {
    pub fn new(x: usize, y: usize, orientation: usize) -> Self {
        let mut visited_positions = HashSet::new();
        visited_positions.insert((x, y));
        Self {
            x,
            y,
            orientation,
            visited_positions,
            is_outside: false,
        }
    }

    pub fn compute_step(&mut self, grid: &Grid) {
        let next_block = self.fetch_next_block(grid);
        if next_block == OBSTRUCTION {
            self.rotate();
        } else if next_block == AVAILABLE_SPOT {
            self.r#move();
        } else if next_block == OUTSIDE {
            self.is_outside = true;
        }
    }

    pub fn fetch_next_block(&self, grid: &Grid) -> char {
        match self.orientation {
            NORTH => grid.get(self.x, self.y.wrapping_sub(1)),
            EAST => grid.get(self.x + 1, self.y),
            SOUTH => grid.get(self.x, self.y + 1),
            WEST => grid.get(self.x.wrapping_sub(1), self.y),
            _ => OUTSIDE,
        }
    }

    fn rotate(&mut self) {
        self.orientation = (self.orientation + 1) % 4;
    }

    fn r#move(&mut self) {
        match self.orientation {
            NORTH => self.y = self.y.wrapping_sub(1),
            EAST => self.x += 1,
            SOUTH => self.y += 1,
            WEST => self.x = self.x.wrapping_sub(1),
            _ => {}
        }
        self.visited_positions.insert((self.x, self.y));
    }

    pub fn unique_positions_count(&self) -> usize {
        self.visited_positions.len()
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    values: Vec<char>,
}

impl Grid {
    pub fn new(width: usize, height: usize, values: Vec<char>) -> Self {
        Self {
            width,
            height,
            values,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        if x < self.width && y < self.height {
            self.values[y * self.width + x]
        } else {
            OUTSIDE
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut values = Vec::new();
    let mut guard_x = 0;
    let mut guard_y = 0;
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.lines().enumerate() {
        width = line.len();
        for (x, ch) in line.chars().enumerate() {
            if ch == '^' {
                guard_x = x;
                guard_y = y;
                values.push(AVAILABLE_SPOT);
            } else {
                values.push(ch);
            }
        }
        height += 1;
    }

    let mut guard = Guard::new(guard_x, guard_y, NORTH);
    let grid = Grid::new(width, height, values);

    while !guard.is_outside {
        guard.compute_step(&grid);
    }

    println!("Distinct positions visited: {}", guard.unique_positions_count());
}
