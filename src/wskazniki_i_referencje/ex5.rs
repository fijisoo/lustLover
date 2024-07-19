/*
Mutowalne referencje:

Napisz program, który używa mutowalnej referencji do zmiany wartości zmiennej.
 */

pub fn run_ex5(){
    let mut x = String::from("test");
    let z = x.clone();
    x = String::from("nowa wartość");

    for i in 0..5 {
        x = String::new() + &x + &i.to_string();
    }

    println!("stara wartosc x: {}", z);;
    println!("nowa wartość x: {}", x);
}