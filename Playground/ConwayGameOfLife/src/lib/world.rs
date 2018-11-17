pub struct World {
    pub width: u16,
    pub height: u16,
    grid: Vec<u8>
}

impl World {

    pub fn new(width: u16, height: u16) -> World {
        World {width, height, grid: Vec::with_capacity((width * height) as usize)}
    }

    pub fn evolve(&mut self) {}

    fn neighbors(&self, (row, col): (u16, u16)) -> Vec<(u16, u16)> {
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
    let neighbors = world.neighbors((1, 1));

    // Assert
    assert_eq!(3, neighbors.len())
}