/*
Referencje:

Napisz program, który deklaruje zmienną i wyświetla jej wartość oraz adres w pamięci za pomocą referencji.
 */

pub fn run_ex4() {
    let x = String::from("String test");

    println!("Wartosc, {}", x);
    println!("Adres pamięci referencji: {:?}", &x as *const String);
    println!("Adres pamięci danych: {:?}", x.as_ptr());
}