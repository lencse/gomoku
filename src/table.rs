const SIZE: usize = 15;

pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn pos(row: usize, col: usize) -> Position {
        assert!(row < SIZE);
        assert!(col < SIZE);
        return Position { row, col };
    }
}

pub struct Table {
    cells: [[u8; SIZE]; SIZE],
}

impl Table {
    pub fn new() -> Table {
        return Table {
            cells: [[0; SIZE]; SIZE],
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
        assert_eq!(0, self.val(&pos));
        assert!(1 == value || 2 == value);
        self.cells[pos.row][pos.col] = value;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[should_panic]
    fn position_check() {
        super::Position::pos(30, 30);
    }

    #[test]
    fn table_creation() {
        let t = super::Table::new();
        assert_eq!(0, t.val(&super::Position::pos(0, 0)));
    }

    #[test]
    fn put_value_to_table() {
        let mut t = super::Table::new();
        let p = &super::Position::pos(5, 7);
        t.put(p, 1);
        assert_eq!(1, t.val(p));
    }

    #[test]
    #[should_panic]
    fn cant_put_on_the_same_cell() {
        let mut t = super::Table::new();
        let p = &super::Position::pos(5, 7);
        t.put(p, 1);
        t.put(p, 2);
    }

    #[test]
    #[should_panic]
    fn cant_put_invalid_value() {
        let mut t = super::Table::new();
        let p = &super::Position::pos(5, 7);
        t.put(p, 3);
    }

}
