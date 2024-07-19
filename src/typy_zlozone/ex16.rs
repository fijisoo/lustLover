/*
Napisz funkcję, która przyjmuje dwa argumenty: licznik i mianownik. Funkcja powinna zwracać wynik dzielenia jako Result<f64, String>, obsługując przypadek dzielenia przez zero.
 */

fn divide_numbers (nominator: f32, denominator: f32) -> Result<f64, String> {
    if(denominator == 0.0){
        Err(format!("you cant divide by 0"))
    } else {
        let result = (nominator/denominator) as f64;
        Ok(result)
    }
}

pub fn run_ex16(){
    let result = match divide_numbers(10f32, 3f32) {
        Ok(value) => format!("result is: {}", value),
        Err(err_message) => err_message
    };

    println!("{}", result)
}

//wynik to 3.3333332538604736 ze względu na konwersje typów oraz błąd zaokrąglenia wynikający z ograniczonej precyzji typów zmiennoprzecinkowych