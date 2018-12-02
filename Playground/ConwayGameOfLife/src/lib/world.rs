#[allow(dead_code)]
extern crate rand;

use self::rand::{Rng};

pub struct World {
    pub width: u32,
    pub height: u32,
    grid: Vec<u8>
}

impl World {
    pub fn new(width: u32, height: u32) -> World {
        World { width, height, grid: vec![0; (width * height) as usize] }
    }

    pub fn init_random(&mut self) {
        let mut rng = rand::thread_rng();
        for row in 0..self.height {
            for col in 0..self.width {
                let rnd = rng.gen_range(0, 100);
                if rnd > 50 {
                    self.set_dead(row, col);
                } else {
                    self.set_alive(row, col);
                }
            }
        }
    }

    pub fn blinker_period_2(&mut self, row: u32, col: u32) {
        assert!(row < self.height);
        assert!(col < self.width);
        let cells = vec![(0, 0), (0, 1), (0, 2)];
        for (r, c) in cells {
            let index = self.index(row + r, col + c);
            self.grid[index] = 1;
        }
    }

    pub fn pentadecathlon(&mut self, row: u32, col: u32) {
        assert!(row < self.height);
        assert!(col < self.width);
        let cells: Vec<(i32, i32)> = vec![(0, 0), (0, 1), (-1, 2), (1, 2), (0, 3), (0, 4), (0, 5), (0, 6), (-1, 7), (1, 7), (0, 8), (0, 9)];
        for (r, c) in cells {
            let rn = row as i32 + r;
            let cn = col as i32 + c;
            let index = self.index(rn as u32, cn as u32);
            self.grid[index] = 1;
        }
    }

    pub fn glider(&mut self, row: u32, col: u32) {
        assert!(row < self.height);
        assert!(col < self.width);
        let cells: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (1, -1), (1, 1)];
        for (r, c) in cells {
            let rn = row as i32 + r;
            let cn = col as i32 + c;
            let index = self.index(rn as u32, cn as u32);
            self.grid[index] = 1;
        }
    }

    pub fn pulsar_period_3(&mut self, row: u32, col: u32) {
        assert!(row < self.height);
        assert!(col < self.width);
        let cells: Vec<(i32, i32)> = vec![
            (-1, -2), (-1, -3), (-1, -4),
            (-2, -1), (-2, -6),
            (-3, -1), (-3, -6),
            (-4, -1), (-4, -6),
            (-6, -2), (-6, -3), (-6, -4),
            (-1, 2), (-1, 3), (-1, 4),
            (-2, 1), (-2, 6),
            (-3, 1), (-3, 6),
            (-4, 1), (-4, 6),
            (-6, 2), (-6, 3), (-6, 4),
            (1, -2), (1, -3), (1, -4),
            (2, -1), (2, -6),
            (3, -1), (3, -6),
            (4, -1), (4, -6),
            (6, -2), (6, -3), (6, -4),
            (1, 2), (1, 3), (1, 4),
            (2, 1), (2, 6),
            (3, 1), (3, 6),
            (4, 1), (4, 6),
            (6, 2), (6, 3), (6, 4),
        ];
        for (r, c) in cells {
            let rn = row as i32 + r;
            let cn = col as i32 + c;
            let index = self.index(rn as u32, cn as u32);
            self.grid[index] = 1;
        }
    }

    pub fn evolve(&self) -> World {
        let mut transformed_world = World::new(self.width, self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                let live_neighbors = self.live_neighbors(row, col);
                if self.is_alive(row, col) {
                    if live_neighbors < 2 {
                        // Any live cell with fewer than two live neighbors dies, as if by underpopulation.
                        transformed_world.set_dead(row, col);
                    } else if live_neighbors == 2 || live_neighbors == 3 {
                        // Any live cell with two or three live neighbors lives on to the next generation.
                        transformed_world.set_alive(row, col);
                    } else if live_neighbors > 3 {
                        // Any live cell with more than three live neighbors dies, as if by overpopulation.
                        transformed_world.set_dead(row, col);
                    }
                } else {
                    if live_neighbors == 3 {
                        // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
                        transformed_world.set_alive(row, col);
                        continue;
                    }
                }
            }
        }
        transformed_world
    }

    fn live_neighbors(&self, row: u32, col: u32) -> usize {
        let neighbors = self.neighbors(row, col);
        neighbors.iter().filter(|(r, c)| self.is_alive(*r, *c)).count()
    }

    fn index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn set_alive(&mut self, row: u32, col: u32) {
        assert!(row < self.height);
        assert!(col < self.width);
        let index = self.index(row, col);
        self.grid[index] = 1;
    }

    pub fn is_alive(&self, row: u32, col: u32) -> bool {
        assert!(row < self.height);
        assert!(col < self.width);
        let index = self.index(row, col);
        self.grid[index] == 1
    }

    fn set_dead(&mut self, row: u32, col: u32) {
        assert!(row < self.height);
        assert!(col < self.width);
        let index = self.index(row, col);
        self.grid[index] = 0;
    }

    fn neighbors(&self, row: u32, col: u32) -> Vec<(u32, u32)> {
        let mut cells = Vec::with_capacity(8);
        let mut rows = vec![row];
        if row > 0 {
            rows.push(row - 1);
        }
        if row < self.height - 1 {
            rows.push(row + 1);
        }
        let mut cols = vec![col];
        if col > 0 {
            cols.push(col - 1);
        }
        if col < self.width - 1 {
            cols.push(col + 1);
        }
        for r in &rows {
            for c in &cols {
                if *r == row && *c == col {
                    continue;
                }
                cells.push((*r, *c));
            }
        }
        cells
    }
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_neighbors() {
        // Arrange
        let world = World::new(2, 2);

        // Act
        let neighbors = world.neighbors(1, 1);

        // Assert
        assert_eq!(3, neighbors.len());
        assert_eq!(Option::Some(&(0, 0)), neighbors.iter().find(|(r, c)| *r == 0 && *c == 0));
        assert_eq!(Option::Some(&(0, 1)), neighbors.iter().find(|(r, c)| *r == 0 && *c == 1));
        assert_eq!(Option::Some(&(1, 0)), neighbors.iter().find(|(r, c)| *r == 1 && *c == 0));
    }

    #[test]
    fn test_set_alive() {
        // Arrange
        let mut world = World::new(2, 2);

        // Act
        world.set_alive(1, 1);

        // Assert
        assert_eq!(1, world.grid[3]);
    }

    #[test]
    fn test_set_dead() {
        // Arrange
        let mut world = World::new(2, 2);

        // Act
        world.set_alive(1, 1);
        world.set_dead(1, 1);

        // Assert
        assert_eq!(0, world.grid[3]);
    }
}
