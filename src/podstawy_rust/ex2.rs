/*
Krotki i tablice:

Napisz program, który tworzy krotkę zawierającą różne typy danych (i32, f64, char) i tablicę liczb całkowitych. Wyświetl wartości krotki i tablicy.
 */

pub fn run_ex2() {
    let krotka: (i32, f64, char) = (12, 2.2, '3');
    let tablica: [u8; 4] = [0,1,2,3];

    println!("result krotka {:?}", krotka);
    println!("result tablica {:?}", tablica);
}
