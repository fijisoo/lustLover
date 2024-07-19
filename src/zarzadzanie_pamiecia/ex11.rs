/*
Napisz program, który używa Box do alokacji liczby całkowitej na stercie. Zmodyfikuj i wyświetl wartość.
 */

pub fn run_ex11(){
    let mut heapInt: Box<u8> = Box::new(12);
    println!("heap:{}", heapInt);
    println!("deref heap:{}", *heapInt);

    *heapInt = 2u8 + *heapInt;

    println!("sum heap:{}", heapInt)
}