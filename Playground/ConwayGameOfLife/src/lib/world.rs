#[warn(dead_code)]
pub struct World {
    pub width: u16,
    pub height: u16,
    grid: Vec<u8>
}

impl World {
    pub fn new(width: u16, height: u16) -> World {
        World { width, height, grid: vec![0; (width * height) as usize] }
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
                    }
                    else if live_neighbors == 2 || live_neighbors == 3 {
                        // Any live cell with two or three live neighbors lives on to the next generation.
                        transformed_world.set_alive(row, col);
                    }
                    else if live_neighbors > 3 {
                        // Any live cell with more than three live neighbors dies, as if by overpopulation.
                        transformed_world.set_dead(row, col);
                    }
                }
                else {
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

    fn live_neighbors(&self, row: u16, col: u16) -> usize {
        let neighbors = self.neighbors(row, col);
        neighbors.iter().filter(|(r, c)| self.is_alive(*r, *c)).count()
    }

    fn index(&self, row: u16, col: u16) -> usize {
        (row * self.width + col) as usize
    }

    fn set_alive(&mut self, row: u16, col: u16) {
        assert!(row < self.height);
        assert!(col < self.width);
        let index = self.index(row, col);
        self.grid[index] = 1;
    }

    fn is_alive(&self, row: u16, col: u16) -> bool {
        assert!(row < self.height);
        assert!(col < self.width);
        let index = self.index(row, col);
        self.grid[index] == 1
    }

    fn set_dead(&mut self, row: u16, col: u16) {
        assert!(row < self.height);
        assert!(col < self.width);
        let index = self.index(row, col);
        self.grid[index] = 0;
    }

    fn neighbors(&self, row: u16, col: u16) -> Vec<(u16, u16)> {
        let mut cells = Vec::with_capacity(8);
        cells.push((row - 1, col - 1));
        cells.push((row - 1, col));
        cells.push((row - 1, col + 1));
        cells.push((row, col - 1));
        cells.push((row, col + 1));
        cells.push((row + 1, col - 1));
        cells.push((row + 1, col));
        cells.push((row + 1, col + 1));
        cells.into_iter().filter(|(r, c)| *r < self.height && *c < self.width).collect()
    }
}

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
