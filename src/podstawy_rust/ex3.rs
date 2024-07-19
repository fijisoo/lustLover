/*
Funkcje:

Napisz funkcję, która przyjmuje dwie liczby całkowite jako argumenty i zwraca ich sumę. Wywołaj tę funkcję i wyświetl wynik.
 */

fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn run_ex3 () {
    let sum = sum(1,2);
    println!("Result {}", sum);
}