/*
Napisz program, który używa surowego wskaźnika (*const i32) do wyświetlenia adresu zmiennej. Skorzystaj z bloku unsafe.
 */

pub fn run_ex12() {
    let x: i32 = 42;
    let x_ptr: *const i32 = &x as *const i32;

    unsafe {
        println!("Wartosc x: {}", x);
        println!("Adres x: {:p}", x_ptr);
        println!("Wartosc pod adresem x_ptr: {}", *x_ptr);
    }
}