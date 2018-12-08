mod game;

fn main() {
    let p = game::Position::pos(3, 5);
    let mut t1 = game::Table::new();
    t1.put(&p, 1);
    t1.put(&game::Position::pos(2, 1), 2);

    println!("({}, {})", t1.val(&p), t1.val(&game::Position::pos(2, 1)));
}
