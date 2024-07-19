/*
Napisz program, który używa zamknięcia (closure) do dodawania dwóch liczb. Wywołaj zamknięcie i wyświetl wynik.
 */

pub fn run_ex17(){
    let z = 1i8;

    let x = |a: i8| -> i8{
        a + z
    };

    println!("wynik: {}", x(2))
}