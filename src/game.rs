const TABLE_SIZE: usize = 15;

pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn pos(row: usize, col: usize) -> Position {
        assert!(row < TABLE_SIZE);
        assert!(col < TABLE_SIZE);
        return Position { row, col };
    }
}

pub struct Table {
    cells: [[u8; TABLE_SIZE]; TABLE_SIZE]
}

impl Table {
    pub fn new() -> Table {
        return Table{
            cells: [[0; TABLE_SIZE]; TABLE_SIZE]
        };
    }
}

impl Table {
    pub fn val(&self, pos: &Position) -> u8 {
        return self.cells[pos.row][pos.col];
    }
}

impl Table {
    pub fn put(&mut self, pos: &Position, value: u8) {
        assert!(0 == self.val(&pos));
        self.cells[pos.row][pos.col] = value;
    }
}
