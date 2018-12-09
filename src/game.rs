mod table;

pub fn pos(row: usize, col: usize) -> table::Position {
    return table::Position::pos(row, col);
}

pub struct Game {
    table: table::Table,
    next: u8,
}

impl Game {
    pub fn new() -> Game {
        return Game {
            table: table::Table::new(),
            next: 1,
        };
    }
}

impl Game {
    pub fn val(&self, pos: &table::Position) -> u8 {
        return self.table.val(pos);
    }
}

impl Game {
    pub fn step(&mut self, pos: &table::Position) {
        self.table.put(pos, self.next);
        self.next = 3 - self.next;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn init() {
        let g = super::Game::new();
        assert_eq!(0, g.val(&super::pos(0, 0)));
    }

    #[test]
    fn step() {
        let mut g = super::Game::new();
        let p1 = &super::pos(0, 0);
        let p2 = &super::pos(1, 0);
        g.step(p1);
        g.step(p2);
        assert_eq!(1, g.val(p1));
        assert_eq!(2, g.val(p2));
    }

}
