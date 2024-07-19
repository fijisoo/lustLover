/*
Napisz funkcję, która zwraca Option<i32>. Funkcja powinna zwracać Some z wartością, jeśli argument jest większy niż 0, lub None w przeciwnym wypadku.
 */

fn return_if_positive (val: i32) -> Option<i32>{
    if(val > 0){
        Some(val)
    } else{
        None
    }
}

pub fn run_ex15(){
    match return_if_positive(2) {
        Some(val) => println!("Is positive!! val: {}", val),
        None => println!("Is negative")
    }

    let test = match return_if_positive(2) {
            Some(val) => format!("Is positive!! val: {}", val),
            None => "Is negative".to_string()
        };

    println!("test: {:?}", test)

}