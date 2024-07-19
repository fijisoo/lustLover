/*
Napisz program, który pokazuje różnicę między przeniesieniem własności (move) a pożyczaniem (borrow) danych. Użyj String do demonstracji.
 */

fn print_text(str: String){
    println!("moved ownership to function: {}", str);
}

pub fn run_ex13(){
    let test_string = String::from("test");
    print_text(test_string); // `test_string` nie jest już dostępny w tym zakresie

    let scalar_type = 8u8;
    let copied_scalar_type = scalar_type; // kopiowanie wartości skalarnej do nowego adresu
    let scalar_type_2 = &scalar_type; // kopiowanie wartości skalarnej, nie powoduje to pożyczania

    println!(
        "scalar type address: {:p}, and copied scalar type address: {:p}. Also scalar_type_2 address: {:p}",
        &scalar_type as *const u8, &copied_scalar_type as *const u8, scalar_type_2 as *const u8
    );

    let reference_type = vec![1, 2, 3, 4];
    let reference_type_2 = reference_type; // przenoszenie własności (move)
    let reference_type_copied = reference_type_2.clone(); // kopiowanie wektora

    println!(
        "reference_type_2 address: {:p}, and copied reference_type address: {:p}",
        &reference_type_2 as *const Vec<i32>, &reference_type_copied as *const Vec<i32>
    );

    let immutable_reference_type_2 = &reference_type_2;
    // let mut mutable_reference_type_2 = &mut reference_type_2; <--- reference_type_2 musi być mutowalne, ale nie jest

    let mut mutable_reference_type_2 = reference_type_2.clone();
    mutable_reference_type_2.push(5);

    println!(
        "reference_type_2 address: {:p}, immutable_reference_type_2 address: {:p}, mutable_reference_type_2 address: {:p}",
        &reference_type_2 as *const Vec<i32>, immutable_reference_type_2 as *const Vec<i32>, &mutable_reference_type_2 as *const Vec<i32>
    );
}