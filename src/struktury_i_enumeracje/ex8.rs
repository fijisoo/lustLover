/*
Enumeracje:

Napisz enumerację Direction z wartościami Up, Down, Left i Right. Napisz funkcję, która przyjmuje Direction jako argument i zwraca odpowiednią wiadomość tekstową.
 */
#[derive(Debug)]
struct Point {
    x: u8,
    y: u8
}
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point)
}

fn event_factory(direction: Direction) {
    match direction {
        Direction::Up(point) => println!("You pressed Up {:?}", point),
        Direction::Down(point) => println!("You pressed Down {:?}", point),
        Direction::Left(point) => println!("You pressed Left {:?}", point),
        Direction::Right(point) => println!("You pressed Right {:?}", point),
    }
}

pub fn run_ex8(){
    event_factory(Direction::Down(Point {x: 10, y: 20}));
}