/*
Struktury:

Napisz strukturę Point z polami x i y typu i32. Utwórz instancję tej struktury i wyświetl jej wartości.
 */
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

pub fn run_ex6(){
    let point1 = Point {
        x: 10,
        y: 20
    };

    println!("wartosc point1: {:?}", point1)

}