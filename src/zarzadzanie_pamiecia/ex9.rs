/*
Napisz program, który tworzy String i dodaje do niego inną wartość tekstową. Wyświetl wynik.
 */

pub fn run_ex9(){
    let a = String::from("String1");
    let b = String::from("String2");
    let c: &str = "string literal";
    let mut d: String;

    d = a + &b + c;

    // dla typów prostych jest copy() dla złożonych clone()
    let mut e = d.clone();

    e = String::from("Who cares");

    println!("{}", d);
    println!("{}", e);
}